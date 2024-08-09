//! collection of structs and utilities for parsing windows binary formats.
#[cfg(feature = "date_time")]
pub mod date_time;
#[cfg(feature = "file_system")]
pub mod file_system;
#[cfg(feature = "structs")]
pub mod structs;
pub mod traits;
#[cfg(feature = "utils")]
pub mod utils;

mod reader_error;
pub use reader_error::*;
