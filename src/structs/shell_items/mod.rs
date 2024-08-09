//! This module contains ShellItems structs parsers.
#![allow(non_camel_case_types)]
#![cfg(feature = "shell_items")]
mod control_panel_category;
mod control_panel_item;
mod file_entry;
mod id_list;
mod network_location;
mod root;
mod uri;
mod users_files_folder;
mod volume;

use control_panel_category::ControlPanelCategoryShellItem;
use control_panel_item::ControlPanelItemShellItem;
use file_entry::FileEntryShellItem;
pub use id_list::IDList;
use network_location::NetworkLocationShellItem;
use root::RootShellItem;
use uri::URIShellItem;
use users_files_folder::UsersFilesFolderShellItem;
use volume::VolumeShellItem;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Cursor, Read, Seek, SeekFrom},
};

use crate::ReaderError;

#[derive(Debug)]
pub struct UnimplementedShellItem(Vec<u8>);

impl Display for UnimplementedShellItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            self.0
                .to_owned()
                .into_iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>()
        )
    }
}

impl Serialize for UnimplementedShellItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// The implemented types for shell items.
#[derive(Debug, Serialize)]
pub enum ShellItemTypes {
    FileEntry(FileEntryShellItem),
    Volume(VolumeShellItem),
    Root(RootShellItem),
    NetworkLocation(NetworkLocationShellItem),
    UsersFilesFolder(UsersFilesFolderShellItem),
    URI(URIShellItem),
    ControlPanelCategory(ControlPanelCategoryShellItem),
    ControlPanelItem(ControlPanelItemShellItem),
    Unimplemented(UnimplementedShellItem),
}

/// ShellItem is struct that reads the struct bytes and decide which shellitem struct to use (FileEntryShellItem, VolumeShellItem, etc).
#[derive(Debug, Serialize)]
pub struct ShellItem {
    /// Shell item size in bytes.
    #[serde(skip_serializing)]
    pub size: u16,
    /// Shell item type (FileEntry, Volume, Root, etc)
    #[serde(skip_serializing)]
    pub class_type: u8,
    /// The parsed shell item data
    pub shell_item_data: Option<ShellItemTypes>,
}

impl ShellItem {
    /// Returns a `ShellItem` from a slice.
    /// # Examples
    ///
    /// ```
    /// use winparsingtools::structs::shell_items::ShellItem;
    ///
    ///     // a buffer that contains the shell item data
    ///     let shell_item_data: &[u8] = &[
    ///        0x5A, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x51, 0x79, 0x62, 0x20, 0x00, 0x74, 0x65,
    ///        0x73, 0x74, 0x2E, 0x74, 0x78, 0x74, 0x00, 0x00, 0x42, 0x00, 0x09, 0x00, 0x04, 0x00, 0xEF, 0xBE,
    ///        0x85, 0x51, 0x79, 0x62, 0x85, 0x51, 0x79, 0x62, 0x2E, 0x00, 0x00, 0x00, 0x26, 0x74, 0x02, 0x00,
    ///        0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///        0x00, 0x00, 0x39, 0xD2, 0x1B, 0x01, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x2E, 0x00,
    ///        0x74, 0x00, 0x78, 0x00, 0x74, 0x00, 0x00, 0x00, 0x18, 0x00
    ///    ];
    ///    println!("{:?}\n",ShellItem::from_buffer(&shell_item_data).unwrap());
    /// ```
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }
    /// Returns a `ShellItem` from an instance that implements `Read` and `Seek`.
    ///
    /// # Examples
    ///
    /// ```
    /// use winparsingtools::structs::shell_items::ShellItem;
    /// use winparsingtools::ReaderError;
    /// use std::fs::File;
    /// // Open a file (std::fs::File implements `Read` and `Seek` traits) then pass it to the function
    /// // to parse the shell item data
    ///
    ///# fn main() -> Result<(), ReaderError> {
    /// let mut shell_item_data = File::open("shell_item_data.bin")?;
    /// println!("{:?}\n",ShellItem::from_reader(&mut shell_item_data).unwrap());
    ///# Ok(())
    ///# }
    /// ```
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        let size = r.read_u16::<LittleEndian>()?;
        let class_type = r.read_u8()?;
        r.seek(SeekFrom::Current(-1))?;
        let mut shell_item_buf = vec![0; (size - 2) as usize];
        r.read_exact(&mut shell_item_buf)?;
        let shell_item_data;

        match class_type {
            class_type if class_type & 0x70 == 0x10 => {
                shell_item_data = Some(ShellItemTypes::Root(RootShellItem::from_buffer(
                    &shell_item_buf,
                )?))
            }
            class_type if class_type & 0x70 == 0x20 => {
                shell_item_data = Some(ShellItemTypes::Volume(VolumeShellItem::from_buffer(
                    &shell_item_buf,
                )?))
            }
            class_type if class_type & 0x70 == 0x30 => {
                shell_item_data = Some(ShellItemTypes::FileEntry(FileEntryShellItem::from_buffer(
                    &shell_item_buf,
                )?))
            }
            class_type if class_type & 0x70 == 0x40 => {
                shell_item_data = Some(ShellItemTypes::NetworkLocation(
                    NetworkLocationShellItem::from_buffer(&shell_item_buf)?,
                ))
            }
            0x74 => {
                shell_item_data = Some(ShellItemTypes::UsersFilesFolder(
                    UsersFilesFolderShellItem::from_buffer(&shell_item_buf)?,
                ))
            }
            0x61 => {
                shell_item_data = Some(ShellItemTypes::URI(URIShellItem::from_buffer(
                    &shell_item_buf,
                )?))
            }
            0x01 => {
                shell_item_data = Some(ShellItemTypes::ControlPanelCategory(
                    ControlPanelCategoryShellItem::from_buffer(&shell_item_buf)?,
                ))
            }
            0x71 => {
                shell_item_data = Some(ShellItemTypes::ControlPanelItem(
                    ControlPanelItemShellItem::from_buffer(&shell_item_buf)?,
                ))
            }
            _ => {
                shell_item_data = Some(ShellItemTypes::Unimplemented(UnimplementedShellItem(
                    shell_item_buf.to_vec(),
                )))
            }
        };

        Ok(Self {
            size,
            class_type,
            shell_item_data,
        })
    }
}

impl Name for ShellItem {
    fn name(&self) -> String {
        match &self.shell_item_data {
            Some(data) => match data {
                ShellItemTypes::Root(item) => item.name(),
                ShellItemTypes::Volume(item) => item.name(),
                ShellItemTypes::FileEntry(item) => item.name(),
                ShellItemTypes::URI(item) => item.name(),
                ShellItemTypes::ControlPanelCategory(item) => item.name(),
                ShellItemTypes::UsersFilesFolder(item) => item.name(),
                ShellItemTypes::ControlPanelItem(item) => item.name(),
                _ => String::from("{NONE}"),
            },
            None => String::from("{NONE}"),
        }
    }
}

/// Extract the data that represents a `name`. This trait is implemented on all shell items to build the path. For example FileEntryShellItem
/// Implements this to return file/directory name. See the implementation of the `Path` trait on `shell_items::IDList` struct for more info.
pub trait Name {
    fn name(&self) -> String;
}
