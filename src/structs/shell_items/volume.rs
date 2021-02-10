use std::io::{Result, Cursor, Read};
use byteorder::ReadBytesExt;
use crate::utils;
use super::Name;
use serde::Serialize;

/// [VolumeShellItem](https://github.com/libyal/libfwsi/blob/main/documentation/Windows%20Shell%20Item%20format.asciidoc#volume_shell_item) struct parser.
#[derive(Debug, Serialize)]
pub struct VolumeShellItem {
    #[serde(skip_serializing)]
    pub has_name: bool,
    pub is_removable_media: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}

impl VolumeShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self>{
        let mut has_name = false;
        let mut is_removable_media = false;
        let class_type = r.read_u8()?; // used to extract flags
        let mut name = None;
        match class_type {
            class_type if class_type & 0x01 == 0x01 => has_name = true,
            class_type if class_type & 0x08 == 0x08 => is_removable_media = true,
            _ => {}
        };
        if !has_name {
            r.read_u8()?; // Remove unknow byte
        }
        else {
            name = match utils::read_utf8_string(r, None) {
                Ok(n) => Some(n),
                Err(_) => None
            };
        }

        Ok(Self {
            has_name,
            is_removable_media,
            name
        })
    }
}

impl Name for VolumeShellItem {
    fn name(&self) -> String {
        match &self.name {
            Some(n) => n.clone(),
            None => String::new()
        }
    }
}
