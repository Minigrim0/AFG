use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use thiserror::Error;

use super::{parser::parse, Instruction};

#[derive(Asset, TypePath, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ProgramLoaderError {
    #[error("Could not load asset: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("Invalid instruction")]
    InvalidInstruction,
}

#[derive(Default)]
pub struct ProgramLoader;

impl AssetLoader for ProgramLoader {
    type Asset = Program;
    type Settings = ();
    type Error = ProgramLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _setting: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text: String = bytes.iter().map(|b| char::from(*b)).collect();
        let instructions = parse(text);
        Ok(Program { instructions })
    }
}
