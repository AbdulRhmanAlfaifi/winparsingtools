use crate::ReaderError;
use crate::structs::{extra_data_block::ExtraDataBlock};
use crate::{
    date_time::FileTime,
    utils::{read_utf16_string, read_utf8_string}
};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Read, Seek, SeekFrom};
use super::Name;

/// [URIShellItem](https://github.com/libyal/libfwsi/blob/main/documentation/Windows%20Shell%20Item%20format.asciidoc#37-uri-shell-item) struct parser.
#[derive(Debug, Serialize)]
pub struct URIShellItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_time: Option<FileTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str3: Option<String>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extention_block: Option<ExtraDataBlock>,
}

impl URIShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        r.read_u8()?; // Remove class_type
        let flags = r.read_u8()?;
        let data_size = r.read_u16::<LittleEndian>()?;
        let mut connect_time = None;
        let mut str1 = None;
        let mut str2 = None;
        let mut str3 = None;
        let uri;
        if data_size > 0 {
            r.seek(SeekFrom::Current(8))?; // Remove unknow bytes
            connect_time = Some(FileTime::new(r.read_u64::<LittleEndian>()?));
            r.seek(SeekFrom::Current(20))?; // Remove unknow bytes
            let str1_size = r.read_u32::<LittleEndian>()?;
            if flags & 0x80 > 0 {
                str1 = Some(read_utf16_string(r, Some(str1_size as usize))?);
            } else {
                str1 = Some(read_utf8_string(r, Some(str1_size as usize))?);
            }
            let str2_size = r.read_u32::<LittleEndian>()?;
            if flags & 0x80 > 0 {
                str2 = Some(read_utf16_string(r, Some(str2_size as usize))?);
            } else {
                str2 = Some(read_utf8_string(r, Some(str2_size as usize))?);
            }
            let str3_size = r.read_u32::<LittleEndian>()?;
            if flags & 0x80 > 0 {
                str3 = Some(read_utf16_string(r, Some(str3_size as usize))?);
            } else {
                str3 = Some(read_utf8_string(r, Some(str3_size as usize))?);
            }
        }

        // Remove null bytes
        loop {
            let byte = r.read_u8()?;
            if byte != 0x00 {
                r.seek(SeekFrom::Current(-1))?;
                break;
            }
        }

        if flags & 0x80 > 0 {
            uri = read_utf16_string(r, None)?;
        } else {
            uri = read_utf8_string(r, None)?;
        }

        r.seek(SeekFrom::Current(2))?; // Remove unknow bytes
        let extention_block = match ExtraDataBlock::from_reader(r) {
            Ok(edb) => edb,
            Err(_) => None
        };

        Ok(Self {
            connect_time,
            str1,
            str2,
            str3,
            uri,
            extention_block
        })
    }
}

impl Name for URIShellItem {
    fn name(&self) -> String {
        self.uri.to_owned()
    }
}
