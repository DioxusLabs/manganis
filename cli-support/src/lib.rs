#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[allow(hidden_glob_reexports)]
mod cache;
mod file;
mod manifest;
mod tailwind;

pub use manganis_common::*;
pub use manifest::*;
pub use tailwind::*;
pub use file::process_file;