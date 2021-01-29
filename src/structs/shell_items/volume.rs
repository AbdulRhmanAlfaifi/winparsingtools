use std::io::{Result, Cursor, Read};
use byteorder::ReadBytesExt;
use crate::utils;
use super::Name;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct VolumeShellItem {
    pub has_name: bool,
    pub is_removable_media: bool,
    pub name: String
}

impl VolumeShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self>{
        let mut has_name = false;
        let mut is_removable_media = false;
        let id = r.read_u8()?;
        match id {
            id if id & 0x01 == 0x01 => has_name = true,
            id if id & 0x08 == 0x08 => is_removable_media = true,
            _ => {}
        };
        if !has_name {
            r.read_u8()?; // Remove unknow byte
        }
        let name = utils::read_utf8_string(r, None)?;

        Ok(Self {
            has_name,
            is_removable_media,
            // type_,
            name
        })
    }
}

impl Name for VolumeShellItem {
    fn name(&self) -> String {
        self.name.clone()
    }
}
