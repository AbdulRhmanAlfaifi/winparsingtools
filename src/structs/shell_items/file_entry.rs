use std::io::{Cursor, Read, SeekFrom, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::date_time::DosDateTime;
use crate::{utils, ReaderError};
use crate::file_system::FileAttributesFlags;
use crate::structs::ExtraDataBlock;
use super::Name;
use serde::Serialize;

/// [FileEntryShellItem](https://github.com/libyal/libfwsi/blob/main/documentation/Windows%20Shell%20Item%20format.asciidoc#file_entry_shell_item) struct parser.
#[derive(Debug, Serialize)]
pub struct FileEntryShellItem{
    pub is_file: bool,
    pub file_size: u32,
    pub mtime: DosDateTime,
    pub file_attr_flags: Vec<FileAttributesFlags>,
    pub name: String,
    pub extention_block: Option<ExtraDataBlock>
}

impl FileEntryShellItem{
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read+Seek>(r: &mut R) -> Result<Self, ReaderError>{
        let class_type = r.read_u8()?;
        let mut is_file = false;
        let mut is_utf16 = false;
        if class_type & 0x2 > 0 {is_file = true};
        if class_type & 0x4 > 0 {is_utf16 = true};
        r.read_u8()?; // remove unknown byte
        let file_size = r.read_u32::<LittleEndian>()?;
        let mtime = DosDateTime::from_u32(r.read_u32::<LittleEndian>()?)?;
        let file_attr_flags = FileAttributesFlags::from_u32(r.read_u16::<LittleEndian>()? as u32);
        let name = match is_utf16{
            true => utils::read_utf16_string(r,None)?,
            false => utils::read_utf8_string(r,None)?
        };
        if !is_utf16 {
            //remove align byte
            loop {
                match r.read_u8() {
                    Ok(byte) => {
                        if byte != 0 {
                            r.seek(SeekFrom::Current(-1))?;
                            break;
                        }
                    },
                    Err(_) => {
                        return Ok(
                            Self{
                                is_file,
                                file_size,
                                mtime,
                                file_attr_flags,
                                name,
                                extention_block: None
                            });
                    }
                };
            }
        }
        let extention_block = ExtraDataBlock::from_reader(r)?;
        Ok(
            Self{
                is_file,
                file_size,
                mtime,
                file_attr_flags,
                name,
                extention_block
            })
    }
}

impl Name for FileEntryShellItem {
    fn name(&self) -> String {
        match &self.extention_block {
            Some(eb) => {
                eb.primary_name.to_owned()
            },
            None => self.name.to_owned() 
        }
    }
}