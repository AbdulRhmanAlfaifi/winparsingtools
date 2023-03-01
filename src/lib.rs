//! collection of structs and utilities for parsing windows binary formats.

pub mod utils;
pub mod date_time;
pub mod file_system;
pub mod structs;
pub mod traits;

mod reader_error;
pub use reader_error::*;
