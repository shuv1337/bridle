//! Configuration management for bridle.

#![allow(dead_code)]
#![allow(unused_imports)]

mod bridle;
mod manager;
mod profile_name;

pub use bridle::{BridleConfig, TuiConfig, ViewPreference};
pub use manager::{McpServerInfo, ProfileInfo, ProfileManager, ResourceSummary};
pub use profile_name::{InvalidProfileName, ProfileName};
