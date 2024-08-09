//! Windows common structs module.
#[cfg(feature = "shell_items")]
mod extra_data_block;
mod guid;
#[cfg(feature = "shell_items")]
pub mod shell_items;
mod string_data;

#[cfg(feature = "shell_items")]
pub use extra_data_block::ExtraDataBlock;
pub use guid::Guid;
pub use string_data::StringData;
