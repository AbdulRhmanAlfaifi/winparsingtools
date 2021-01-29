use std::io::{Cursor, Read, Result, SeekFrom, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::date_time::DosDateTime;
use crate::utils;
use crate::file_system::FileAttributesFlags;
use crate::structs::ExtraDataBlock;
use super::Name;
use serde::Serialize;

/// FileEntryShellItem struct parser.
#[derive(Debug, Serialize)]
pub struct FileEntryShellItem{
    pub is_file: bool,
    pub file_size: u32,
    pub last_modified: DosDateTime,
    pub file_attr_flags: Vec<FileAttributesFlags>,
    pub name: String,
    pub extention_block: Option<ExtraDataBlock>
}

impl FileEntryShellItem{
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read+Seek>(r: &mut R) -> Result<Self>{
        let id = r.read_u8()?;
        let mut is_file = false;
        let mut is_utf16 = false;
        match id & 0x8f {
            0x2 => {is_file = true},
            0x4 => {is_utf16 = true},
            _ => {}
        };
        r.read_u8()?; // remove unknown byte  
        let file_size = r.read_u32::<LittleEndian>()?;
        let mut dos_date_time_bytes = [0;4];
        let last_modified = DosDateTime::from_u32(r.read_u32::<LittleEndian>()?)?;
        let file_attr_flags = FileAttributesFlags::from_u32(r.read_u16::<LittleEndian>()? as u32); 
        let name = match is_utf16{
            true => utils::read_utf16_string(r,Option::None)?,
            false => utils::read_utf8_string(r,Option::None)?
        };
        if !is_utf16 {
            //remove align byte
            if r.read_u8()? != 0 {
                r.seek(SeekFrom::Current(-1))?;
            }
        }
        let extention_block = ExtraDataBlock::from_reader(r)?;
        Ok(
            Self{
                is_file,
                file_size,
                last_modified,
                file_attr_flags,
                name,
                extention_block
            })
    }
}

impl Name for FileEntryShellItem {
    fn name(&self) -> String {
        self.name.clone()
    }
}