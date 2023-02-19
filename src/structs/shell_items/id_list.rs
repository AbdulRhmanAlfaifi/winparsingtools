use crate::ReaderError;
use crate::structs::shell_items::{Name, ShellItem};
use crate::traits::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Read, Seek, SeekFrom};

/// [IDList](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/470e62dc-6c62-49c4-b205-2c39780f10f7) struct parser.
#[derive(Debug, Serialize)]
pub struct IDList(Vec<ShellItem>);

impl IDList {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        let mut item_id_list = vec![];
        loop {
            let size = r.read_u16::<LittleEndian>()?;
            if size == 0 {
                break;
            }
            let mut shell_item_data = vec![0; size as usize];
            r.seek(SeekFrom::Current(-2))?;
            r.read_exact(&mut shell_item_data)?;
            item_id_list.push(ShellItem::from_buffer(&shell_item_data)?);
        }
        Ok(Self(item_id_list))
    }

    pub fn items<'a>(&'a self) -> std::slice::Iter<'a, ShellItem> {
        self.0.iter()
    }
}

impl Path for IDList {
    fn path(&self) -> Option<String> {
        Some(
            self.0
                .iter()
                .map(|shell_item| shell_item.name())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("\\")
                .replace("\\\\", "\\"),
        )
    }
}

