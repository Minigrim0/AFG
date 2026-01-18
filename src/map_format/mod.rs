//! Binary map format for AFG game maps.
//!
//! This module provides structures and utilities for loading/saving game maps
//! in a compact binary format using bincode serialization.

mod loader;
mod structures;

pub use loader::GameMapLoader;
pub use structures::*;
