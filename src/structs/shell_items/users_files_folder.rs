use super::Name;
use crate::structs::{extra_data_block::ExtraDataBlock, guid, shell_items::ShellItem};
use crate::utils::read_utf8_string;
use crate::ReaderError;
use byteorder::{LittleEndian, ReadBytesExt};
use guid::Guid;
use serde::Serialize;
use std::io::{Cursor, Read, Seek, SeekFrom};

/// [UsersFilesFolderShellItem](https://github.com/EricZimmerman/Lnk/tree/master/Lnk/ShellItems) struct parser.
#[derive(Debug, Serialize)]
pub struct UsersFilesFolderShellItem {
    file_entry: Box<Option<ShellItem>>,
    delegate_guid: Guid,
    item_guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    extention_block: Option<ExtraDataBlock>,
}

impl UsersFilesFolderShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, ReaderError> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, ReaderError> {
        r.read_u16::<LittleEndian>()?; // No other data on the class_type
        let size = r.read_u16::<LittleEndian>()?; // remove the size
        let sig = read_utf8_string(r, Some(4))?;
        let mut file_entry = None;
        let mut shell_item_data = vec![0; (size - 6) as usize];
        r.read_exact(&mut shell_item_data)?;
        if sig == "CFSF" {
            file_entry = Some(ShellItem::from_buffer(&shell_item_data)?);
        } else if sig == "CF" {
            // TODO: implement this
            return Err(ReaderError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "UsersFilesFolderShellItem with signitaure 'CF' is not implemented",
            )));
        }

        // Remove null bytes
        loop {
            if r.read_u8()? == 0x00 {
                r.seek(SeekFrom::Current(-1))?;
                break;
            }
        }
        let delegate_guid = Guid::from_reader(r)?;
        if delegate_guid.to_string().to_lowercase() != "5e591a74-df96-48d3-8d67-1733bcee28ba" {
            return Err(ReaderError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "WRONG GUID : Expected '5e591a74-df96-48d3-8d67-1733bcee28ba' found '{}'",
                    delegate_guid.to_string().to_lowercase()
                )),
            ));
        }
        let item_guid = Guid::from_reader(r)?;
        let extention_block = ExtraDataBlock::from_reader(r)?;
        Ok(Self {
            file_entry: Box::new(file_entry),
            delegate_guid,
            item_guid,
            extention_block,
        })
    }
}

impl Name for UsersFilesFolderShellItem {
    fn name(&self) -> String {
        match &*self.file_entry {
            Some(shell_item) => shell_item.name(),
            None => String::from("{NONE}"),
        }
    }
}
