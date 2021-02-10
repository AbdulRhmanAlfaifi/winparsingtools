//! Traits to help standardize parsers output.
use std::collections::HashMap;

/// The `Normalize` trait allow of returing the most important data from struct to be used
/// on simple data structures like CSV.
pub trait Normalize {
    fn normalize(&self) -> HashMap<String, String>;
}

/// Implement this trait to generate the path from deferent structs
pub trait Path {
    fn path(&self) -> Option<String>;
}