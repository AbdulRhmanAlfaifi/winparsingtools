//! Windows common structs module.
pub mod shell_items;
pub mod extra_data_block;
pub mod guid;
pub mod string_data;

pub use extra_data_block::ExtraDataBlock;
pub use guid::Guid;
pub use string_data::StringData;