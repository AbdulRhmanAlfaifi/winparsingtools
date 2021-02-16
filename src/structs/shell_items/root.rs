use std::io::{Result, Cursor, Read};
use byteorder::ReadBytesExt;
use crate::structs::guid;
use super::Name;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RootShellItemSortIndex {
    INTERNET_EXPLORER1 = 0,
    LIBRARIES = 66,
    USERS = 68,
    MY_DOCUMENTS = 72,
    MY_COMPUTER = 80,
    MY_NETWORK_PLACES = 88,
    RECYCLE_BIN = 96,
    INTERNET_EXPLORER2 = 104,
    UNKNOWN = 112,
    MY_GAMES = 128
}

impl From<u8> for RootShellItemSortIndex {
    fn from(num: u8) -> Self {
        match num {
            0 => RootShellItemSortIndex::INTERNET_EXPLORER1,
            66 => RootShellItemSortIndex::LIBRARIES,
            68 => RootShellItemSortIndex::USERS,
            72 => RootShellItemSortIndex::MY_DOCUMENTS,
            80 => RootShellItemSortIndex::MY_COMPUTER,
            88 => RootShellItemSortIndex::MY_NETWORK_PLACES,
            96 => RootShellItemSortIndex::RECYCLE_BIN,
            104 => RootShellItemSortIndex::INTERNET_EXPLORER2,
            128 => RootShellItemSortIndex::MY_GAMES,
            _ => RootShellItemSortIndex::UNKNOWN,
        }
    }
}

/// RootShellItem struct parser.
#[derive(Debug, Serialize)]
pub struct RootShellItem {
    pub sort_index: RootShellItemSortIndex,
    pub guid: guid::Guid
}

impl RootShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self>{
        let _class_type = r.read_u8()?; // used to extract flags
        let sort_index = RootShellItemSortIndex::from(r.read_u8()?);
        let guid = guid::Guid::from_reader(r)?;
        Ok(Self {
            sort_index,
            guid
        })
    }
}

impl Name for RootShellItem {
    fn name(&self) -> String {
        match &self.sort_index {
            RootShellItemSortIndex::UNKNOWN => format!("{}",&self.guid),
            other => format!("{:?}",other)
        }
    }
}