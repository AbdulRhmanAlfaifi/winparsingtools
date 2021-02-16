use crate::structs::{ExtraDataBlock, Guid};
use super::Name;
use serde::Serialize;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};

/// [ControlPanelItemShellItem](https://github.com/EricZimmerman/Lnk/tree/master/Lnk/ShellItems) struct parser.
#[derive(Debug, Serialize)]
pub struct ControlPanelItemShellItem {
    pub guid: Guid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extention_block: Option<ExtraDataBlock>,
}

impl ControlPanelItemShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        r.seek(SeekFrom::Current(12))?;
        let guid = Guid::from_reader(r)?;
        let extention_block = match ExtraDataBlock::from_reader(r) {
            Ok(eb) => eb,
            Err(_) => None,
        };

        Ok(Self {
            guid,
            extention_block,
        })
    }
}

impl Name for ControlPanelItemShellItem {
    fn name(&self) -> String {
        self.guid.to_string()
    }
}
