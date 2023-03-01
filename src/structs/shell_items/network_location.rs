use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::ReadBytesExt;
use crate::{utils::read_utf8_string, ReaderError};
use super::Name;
use serde::Serialize;

/// [NetworkLocationShellItem](https://github.com/libyal/libfwsi/blob/main/documentation/Windows%20Shell%20Item%20format.asciidoc#network_location_shell_item) struct parser.
#[derive(Debug, Serialize)]
pub struct NetworkLocationShellItem {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>

}

impl NetworkLocationShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError>{
        let _class_type = r.read_u8()?; // used to extract flags
        let mut description = None;
        let mut comments = None;
        r.seek(SeekFrom::Current(1))?;
        let flags = r.read_u8()?;

        let location = read_utf8_string(r, None)?;

        // has description
        if flags & 0x80 > 0 {
            description = Some(read_utf8_string(r, None)?);
        }
        
        // has comments
        if flags & 0x40 > 0 {
            comments = Some(read_utf8_string(r, None)?);
        }

        Ok( 
            Self {
                location,
                description,
                comments
            }
        )

    }
}

impl Name for NetworkLocationShellItem {
    fn name(&self) -> String {
        format!("{:?}",&self.location)
    }
}