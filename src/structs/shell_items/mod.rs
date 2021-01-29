pub mod file_entry;
pub mod volume;
pub mod root;

use std::io::{Result, Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
pub use file_entry::FileEntryShellItem;
pub use volume::VolumeShellItem;
pub use root::RootShellItem;
use serde::Serialize;

/// This module contains ShellItems structs parsers.

#[derive(Debug, Serialize)]
enum ShellItemType {
    UNKNWON,
    ROOT,
    VOLUME,
    FILE,
    NETWORK,
    COMPRESSED,
    URI,
    CONTROL_PANEL,
}
impl From<u8> for ShellItemType {
    fn from(num: u8) -> Self {
        match num {
            1 => ShellItemType::ROOT,
            2 => ShellItemType::VOLUME,
            3 => ShellItemType::FILE,
            4 => ShellItemType::NETWORK,
            5 => ShellItemType::COMPRESSED,
            6 => ShellItemType::URI,
            7 => ShellItemType::CONTROL_PANEL,
            _ => ShellItemType::UNKNWON
        }
    }
}


#[derive(Debug,Serialize)]
pub enum ShellItemTypes {
    FileEntry(FileEntryShellItem),
    Volume(VolumeShellItem),
    Root(RootShellItem),
}
#[derive(Debug,Serialize)]
pub struct ShellItem {
    #[serde(skip_serializing)]
    pub size: u16,
    #[serde(skip_serializing)]
    pub class_type: u8,
    pub shell_item_data: Option<ShellItemTypes>
}

impl ShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let size = r.read_u16::<LittleEndian>()?;
        let class_type = r.read_u8()? & 0x70;
        r.seek(SeekFrom::Current(-1))?;
        let mut shell_item_data = None;

        match class_type {
            0x10 => shell_item_data = Some(ShellItemTypes::Root(RootShellItem::from_reader(r)?)),
            0x20 => shell_item_data = Some(ShellItemTypes::Volume(VolumeShellItem::from_reader(r)?)),
            0x30 => shell_item_data = Some(ShellItemTypes::FileEntry(FileEntryShellItem::from_reader(r)?)),
            _ => {}
        };

        Ok(Self {
            size,
            class_type,
            shell_item_data
        })
    }
}


pub trait Name {
    fn name(&self) -> String;
}