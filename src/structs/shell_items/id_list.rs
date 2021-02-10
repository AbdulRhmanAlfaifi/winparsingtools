use std::io::{Result, Read, Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::structs::shell_items::{ShellItem, ShellItemTypes, Name};
use serde::Serialize;
use crate::traits::Path;


/// [IDList](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/470e62dc-6c62-49c4-b205-2c39780f10f7) struct parser.
#[derive(Debug, Serialize)]
pub struct IDList(Vec<ShellItem>);

impl IDList {
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let mut item_id_list = vec![];
        loop {
            let size = r.read_u16::<LittleEndian>()?;
            if size == 0 {
                break;
            }
            let mut shell_item_data = vec![0;size as usize];
            r.seek(SeekFrom::Current(-2))?;
            r.read_exact(&mut shell_item_data)?;
            item_id_list.push(ShellItem::from_buffer(&shell_item_data)?);
        }
        Ok(Self (item_id_list))
    }
}

impl Path for IDList {
    fn path(&self) -> Option<String> {
        Some(
            self.0.iter().map(|shell_item| match &shell_item.shell_item_data {
            Some(data) => {
                match data {
                    ShellItemTypes::Root(item) => {
                        let name = item.name();
                        if name == "MY_COMPUTER" {
                            return String::new();
                        }
                        name
                    },
                    ShellItemTypes::Volume(item) => {item.name()},
                    ShellItemTypes::FileEntry(item) => {item.name()},
                    _ => String::from("{NONE}")
                }
            },
            None => String::from("{NONE}")
        }).filter(|s| !s.is_empty()).collect::<Vec<String>>().join("\\").replace("\\\\", "\\")
        )
    }
}