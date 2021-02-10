use std::io::{Result, Cursor, Read};
use std::fmt::{Formatter, Display, Result as FmtResult};
use byteorder::{ReadBytesExt, LittleEndian};
use serde::Serialize;

/// `FileReference` struct parser.`FileReference` is a struct that contains the file `mft_entry` and `sequence_number`.
#[derive(Debug, Serialize)]
pub struct FileReference {
    mft_entry: u64,
    sequence_number: u16
}

impl FileReference{
    /// Create FileReference from bytes.
    pub fn from_buffer(buf: &[u8]) -> Result<FileReference>{
        Self::from_reader(&mut Cursor::new(buf))
    }
    
    /// Create FileReference from instent that implements `Read` trait.
    pub fn from_reader<R: Read>(r: &mut R) -> Result<FileReference>{
        let mut mft_entry_bytes = [0;6];
        r.read_exact(&mut mft_entry_bytes).unwrap();
        let mut bytes = mft_entry_bytes.to_vec();
        bytes.push(0);
        bytes.push(0);
        let mft_entry = Cursor::new(bytes).read_u64::<LittleEndian>().unwrap();
        let sequence_number = r.read_u16::<LittleEndian>().unwrap();
        Ok(
            FileReference{
                mft_entry,
                sequence_number
            }
        )
    }
}

/// [File system attributes flags](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/ca28ec38-f155-4768-81d6-4bfeb8586fc9) parser.
#[derive(Debug, Serialize)]
pub enum FileAttributesFlags{
    READONLY,
    HIDDEN,
    SYSTEM,
    VOLUME_LABEL,
    DIRECTORY,
    ARCHIVE,
    DEVICE,
    NORMAL,
    TEMPORARY,
    SPARSE_FILE,
    REPARSE_POINT,
    COMPRESSED,
    OFFLINE,
    NOT_CONTENT_INDEXED,
    ENCRYPTED,
    INTEGRITY_STREAM,
    VIRTUAL,
    NO_SCRUB_DATA
}

impl FileAttributesFlags {
    pub fn from_u32(flags: u32) -> Vec<FileAttributesFlags>{
        let mut res = vec![];
        if flags & 0x00000001 !=0 { res.push(FileAttributesFlags::READONLY); }
        if flags & 0x00000002 !=0 { res.push(FileAttributesFlags::HIDDEN); }
        if flags & 0x00000004 !=0 { res.push(FileAttributesFlags::SYSTEM); }
        if flags & 0x00000008 !=0 { res.push(FileAttributesFlags::VOLUME_LABEL); }
        if flags & 0x00000010 !=0 { res.push(FileAttributesFlags::DIRECTORY); }
        if flags & 0x00000020 !=0 { res.push(FileAttributesFlags::ARCHIVE); }
        if flags & 0x00000040 !=0 { res.push(FileAttributesFlags::DEVICE); }
        if flags & 0x00000080 !=0 { res.push(FileAttributesFlags::NORMAL); }
        if flags & 0x00000100 !=0 { res.push(FileAttributesFlags::TEMPORARY); }
        if flags & 0x00000200 !=0 { res.push(FileAttributesFlags::SPARSE_FILE); }
        if flags & 0x00000400 !=0 { res.push(FileAttributesFlags::REPARSE_POINT); }
        if flags & 0x00000800 !=0 { res.push(FileAttributesFlags::COMPRESSED); }
        if flags & 0x00001000 !=0 { res.push(FileAttributesFlags::OFFLINE); }
        if flags & 0x00002000 !=0 { res.push(FileAttributesFlags::NOT_CONTENT_INDEXED); }
        if flags & 0x00004000 !=0 { res.push(FileAttributesFlags::ENCRYPTED); }
        if flags & 0x00008000 !=0 { res.push(FileAttributesFlags::INTEGRITY_STREAM); }
        if flags & 0x00010000 !=0 { res.push(FileAttributesFlags::VIRTUAL); }
        if flags & 0x00020000 !=0 { res.push(FileAttributesFlags::NO_SCRUB_DATA); }
        res
    }
}

impl Display for FileAttributesFlags{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}",self)
    }
}