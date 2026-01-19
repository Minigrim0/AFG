//! Bevy asset loader for GameMap files.

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use thiserror::Error;

use super::{GameMap, MAP_FILE_EXTENSION};

/// Errors that can occur when loading a GameMap.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum GameMapLoaderError {
    #[error("Could not read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to decode map data: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Invalid map version: expected {expected}, got {got}")]
    VersionMismatch { expected: u32, got: u32 },
}

/// Asset loader for binary GameMap files.
#[derive(Default)]
pub struct GameMapLoader;

impl AssetLoader for GameMapLoader {
    type Asset = GameMap;
    type Settings = ();
    type Error = GameMapLoaderError;

    fn extensions(&self) -> &[&str] {
        &[MAP_FILE_EXTENSION]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let map = GameMap::from_bytes(&bytes)?;

        // Version check - can add migration logic here in the future
        if map.version > super::MAP_FORMAT_VERSION {
            return Err(GameMapLoaderError::VersionMismatch {
                expected: super::MAP_FORMAT_VERSION,
                got: map.version,
            });
        }

        Ok(map)
    }
}

/// Plugin that registers the GameMap asset loader.
pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameMap>()
            .init_asset_loader::<GameMapLoader>();
    }
}

/// Utility functions for saving maps to disk.
impl GameMap {
    /// Saves the map to a file.
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let bytes = self
            .to_bytes()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, bytes)
    }

    /// Loads a map from a file (synchronous, for editor use).
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, GameMapLoaderError> {
        let bytes = std::fs::read(path)?;
        Ok(Self::from_bytes(&bytes)?)
    }
}
