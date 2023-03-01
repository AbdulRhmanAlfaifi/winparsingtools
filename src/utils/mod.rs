//! Utilites used for formating data.

mod rot13;
use encoding_rs::WINDOWS_1252;
pub use rot13::Rot13;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};
use std::{char::decode_utf16, str::from_utf8};

use crate::ReaderError;
// https://github.com/omerbenamram/mft/blob/master/src/utils.rs
/// Read UTF-16LE string from a stream and return it as `String`.
pub fn read_utf16_string<T: Read>(
    stream: &mut T,
    len: Option<usize>,
) -> Result<String, ReaderError> {
    let mut buffer = match len {
        Some(len) => Vec::with_capacity(len),
        None => Vec::new(),
    };

    match len {
        Some(len) => {
            for _ in 0..len {
                let next_char = stream.read_u16::<LittleEndian>()?;
                buffer.push(next_char);
            }
        }
        None => loop {
            let next_char = stream.read_u16::<LittleEndian>()?;

            if next_char == 0 {
                break;
            }

            buffer.push(next_char);
        },
    }

    // We need to stop if we see a NUL byte, even if asked for more bytes.
    decode_utf16(buffer.into_iter().take_while(|&byte| byte != 0x00))
        .map(|r| r.map_err(ReaderError::from))
        .collect()
}

/// Read UTF-8 string from a stream and return it as `String`.
pub fn read_utf8_string<R: Read>(
    stream: &mut R,
    len: Option<usize>,
) -> Result<String, ReaderError> {
    let mut buffer = match len {
        Some(len) => Vec::with_capacity(len),
        None => Vec::new(),
    };

    match len {
        Some(len) => {
            for _ in 0..len {
                let next_char = stream.read_u8()?;
                buffer.push(next_char);
            }
        }
        None => loop {
            let next_char = stream.read_u8()?;

            if next_char == 0 {
                break;
            }

            buffer.push(next_char);
        },
    }
    from_utf8(
        buffer
            .into_iter()
            .take_while(|&byte| byte != 0x00)
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .map_err(ReaderError::from)
    .map(|r| r.to_string())
}

/// read a single byte string in some unknown code page.
/// Can this happen? Yes. Consider <https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-SHLLINK/%5bMS-SHLLINK%5d.pdf>
/// which states things like `A NULL–terminated string, defined by the system default code page`
/// 
/// no words...
/// 
/// This method assumes CP1252, which is common for a lot of systems, and maps invalid characters to '?'
/// 
pub fn read_cp1252_string<R: Read>(
    stream: &mut R,
    len: Option<usize>,
) -> Result<String, ReaderError> {
    let mut buffer = match len {
        Some(len) => Vec::with_capacity(len),
        None => Vec::new(),
    };

    match len {
        Some(len) => {
            for _ in 0..len {
                let next_char = stream.read_u8()?;
                buffer.push(next_char);
            }
        }
        None => loop {
            let next_char = stream.read_u8()?;

            if next_char == 0 {
                break;
            }

            buffer.push(next_char);
        },
    }
    let (cow, _encoding_used, had_errors) = WINDOWS_1252.decode(&buffer[..]);
    if had_errors {
        Err(ReaderError::CP1252Error)
    } else {
        Ok(cow.into())
    }
}
