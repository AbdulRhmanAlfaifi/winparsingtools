#![allow(non_camel_case_types)]
//! This module contains ShellItems structs parsers.

mod file_entry;
mod volume;
mod root;
mod network_location;
mod id_list;

use file_entry::FileEntryShellItem;
use volume::VolumeShellItem;
use root::RootShellItem;
use network_location::NetworkLocationShellItem;
pub use id_list::IDList;

use std::io::{Result, Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;

/// The implemented types for shell items.
#[derive(Debug,Serialize)]
pub enum ShellItemTypes {
    FileEntry(FileEntryShellItem),
    Volume(VolumeShellItem),
    Root(RootShellItem),
    NetworkLocation(NetworkLocationShellItem),
    Unimpleminted(String),
}

/// ShellItem is struct that reads the struct bytes and decide which shellitem struct to use (FileEntryShellItem, VolumeShellItem, etc).
#[derive(Debug,Serialize)]
pub struct ShellItem {
    /// Shell item size in bytes.
    #[serde(skip_serializing)]
    pub size: u16,
    /// Shell item type (FileEntry, Volume, Root, etc)
    #[serde(skip_serializing)]
    pub class_type: u8,
    /// The parsed shell item data
    pub shell_item_data: Option<ShellItemTypes>
}

impl ShellItem {
    /// Returns a `ShellItem` from a slice.
    /// # Examples
    ///
    /// ```
    /// use winparsingtools::structs::shell_items::ShellItem;
    /// fn main (){
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
    /// }
    /// ```
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }
    /// Returns a `ShellItem` from an instence that implements `Read` and `Seek`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Open a file (std::fs::File implements `Read` and `Seek` traits) then pass it to the function
    /// // to parse the shell item data
    /// let mut shell_item_data = File::open("shell_item_data.bin")?;
    ///println!("{:?}\n",ShellItem::from_reader(&mut shell_item_data).unwrap());
    /// ```
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let size = r.read_u16::<LittleEndian>()?;
        let class_type = r.read_u8()? & 0x70;
        r.seek(SeekFrom::Current(-1))?;
        let mut shell_item_buf = vec![0;(size - 2) as usize];
        r.read_exact(&mut shell_item_buf)?;
        let shell_item_data;

        match class_type {
            0x10 => shell_item_data = Some(ShellItemTypes::Root(RootShellItem::from_buffer(&shell_item_buf)?)),
            0x20 => shell_item_data = Some(ShellItemTypes::Volume(VolumeShellItem::from_buffer(&shell_item_buf)?)),
            0x30 => shell_item_data = Some(ShellItemTypes::FileEntry(FileEntryShellItem::from_buffer(&shell_item_buf)?)),
            0x40 => shell_item_data = Some(ShellItemTypes::NetworkLocation(NetworkLocationShellItem::from_buffer(&shell_item_buf)?)),
            _ => shell_item_data = Some(ShellItemTypes::Unimpleminted(String::from("Unimplemented ShellItem")))
        };

        Ok(Self {
            size,
            class_type,
            shell_item_data
        })
    }
}

/// Extract the data that represents a `name`. This trait is implemented on all shell items to build the path. For example FileEntryShellItem
/// Implements this to return file/directory name. See the implementation of the `Path` trait on `shell_items::IDList` struct for more info.
pub trait Name {
    fn name(&self) -> String;
}