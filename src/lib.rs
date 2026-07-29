//! This is a module-level comment for a Rust lib

#![cfg_attr(not(test), deny(unused_crate_dependencies))]

mod command;

pub use command::*;
