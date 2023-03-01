use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::ReaderError;
use crate::date_time::DosDateTime;
use crate::utils::{read_utf16_string, read_utf8_string};
use crate::file_system::FileReference;
use serde::Serialize;

/// [ExtraDataBlock (BEEF0004)](https://github.com/libyal/libfwsi/blob/main/documentation/Windows%20Shell%20Item%20format.asciidoc#extension_block_0xbeef0004) struct parser.
#[derive(Debug, Serialize)]
pub struct ExtraDataBlock {
    #[serde(skip_serializing)]
    pub size: u16,
    #[serde(skip_serializing)]
    pub version: u16,
    #[serde(skip_serializing)]
    pub signature: [u8;4],
    pub ctime: DosDateTime,
    pub atime: DosDateTime,
    #[serde(skip_serializing)]
    pub identifier: u16,
    pub file_ref: Option<FileReference>,
    #[serde(skip_serializing)]
    pub long_str_size: u16,
    pub primary_name: String,
    #[serde(skip_serializing)]
    pub version_offset: Option<u16>
}

impl ExtraDataBlock{
    pub fn from_buffer(buf: &[u8]) -> Result<Option<ExtraDataBlock>, ReaderError>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Option<ExtraDataBlock>, ReaderError>{
        let size =  r.read_u16::<LittleEndian>()?;
        if size == 0 {
            return Ok(None);
        }
        let version =  r.read_u16::<LittleEndian>()?;
        let mut signature = [0;4];
        r.read_exact(&mut signature)?;
        let ctime = DosDateTime::from_u32(r.read_u32::<LittleEndian>()?)?;
        let atime = DosDateTime::from_u32(r.read_u32::<LittleEndian>()?)?;
        let identifier =  r.read_u16::<LittleEndian>()?;
        let file_ref = match version {
            _v if version >= 7 => {
                r.read_u16::<LittleEndian>()?;
                let mut file_ref_bytes = [0;8];
                r.read_exact(&mut file_ref_bytes)?;
                let mut unknown = [0;8];
                r.read_exact(&mut unknown)?; // ignore unkwon 8 bytes
                Some(FileReference::from_buffer(&file_ref_bytes)?)
            },
            _ => Option::None
        };
        let long_str_size = match version {
            _v if version >= 3 => {
                r.read_u16::<LittleEndian>()?
            },
            _ => 0
        };

        // ignore unkwon bytes
        match version {
            _v if version >= 8 => {
                r.read_u64::<LittleEndian>()?;
                0
            },
            _v if version >= 9 => {
                r.read_u32::<LittleEndian>()?;
                0
            },
            _ => 0
        };

        let primary_name = match version {
            _v if version >= 7 && long_str_size == 0 => {
                read_utf16_string(r, Option::None)?
            },
            _v if version >= 3 && long_str_size == 0 => {
                read_utf8_string(r, Option::None)?
            },
            _ => read_utf8_string(r, Option::None)?
        };

        let version_offset = match version {
            _v if version >= 3 => {
                Some(r.read_u16::<LittleEndian>()?)
            },
            _ => Option::None
        };

        Ok(Some(
            ExtraDataBlock {
                size,
                version,
                signature,
                ctime,
                atime,
                identifier,
                file_ref,
                long_str_size,
                primary_name,
                version_offset,
            })
        )
    }
}