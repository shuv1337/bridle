//! Installation management for bridle.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod discovery;
mod types;

pub use discovery::{discover_skills, DiscoveryError};
pub use types::*;
