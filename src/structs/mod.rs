//! Windows common structs module.
pub mod shell_items;
mod extra_data_block;
mod guid;
mod string_data;

pub use extra_data_block::ExtraDataBlock;
pub use guid::Guid;
pub use string_data::StringData;