use std::io::{Cursor, Read, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Result as FmtResult, Display, Formatter};
use serde::ser;

/// [GUID](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid) struct parser.
/// From [omerbenamram](https://github.com/omerbenamram/winstructs/blob/master/src/guid.rs)
#[derive(Debug, Clone)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    /// Creates a new GUID directly from it's components.
    pub fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Reads a GUID from a buffer.
    pub fn from_buffer(buffer: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buffer))
    }

    /// Reads a GUID from a `Read` instance.
    pub fn from_reader<T: Read>(stream: &mut T) -> Result<Self> {
        let data1 = stream.read_u32::<LittleEndian>()?;
        let data2 = stream.read_u16::<LittleEndian>()?;
        let data3 = stream.read_u16::<LittleEndian>()?;

        let mut data4 = [0; 8];
        stream.read_exact(&mut data4)?;

        Ok(Self {
            data1,
            data2,
            data3,
            data4
        })
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

/// For GUIDs, a string representation is preferable to a struct for serialization.
impl ser::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}