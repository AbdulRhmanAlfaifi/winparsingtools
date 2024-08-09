//! Contains file system related windows structs.
#![allow(non_camel_case_types)]
#![cfg(feature = "file_system")]
mod file_reference;

pub use file_reference::FileAttributesFlags;
pub use file_reference::FileReference;
