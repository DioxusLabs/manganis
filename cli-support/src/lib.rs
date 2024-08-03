#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[allow(hidden_glob_reexports)]
mod file;
mod linker_intercept;
mod manifest;

pub use file::process_file;
pub use linker_intercept::*;
pub use manganis_common::*;
pub use manifest::*;
