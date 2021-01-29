use std::io::{Read, Result, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Formatter, Display, Result as FmtResult};
use crate::utils::read_utf16_string;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct StringData {
    size: u16,
    string: String
}

impl StringData {
    pub fn from_buffer(buf: &[u8]) -> Result<Self>{
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self>{
        let size = r.read_u16::<LittleEndian>()?;
        let string = read_utf16_string(r, Some(size as usize))?;

        Ok(Self {
            size,
            string
        })
    }
}

impl Display for StringData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.string)
    }
}

impl Serialize for StringData {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}