use super::StyleSheetAsset;

use bevy::{
    asset::{
        io::Reader,
        AssetLoader, AsyncReadExt,
        LoadContext,
    },
    prelude::*
};
use thiserror::Error;

#[derive(Default)]
pub(crate) struct StyleSheetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub(crate) enum StyleSheetLoaderError
{
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse file: {0}")]
    Parsing(#[from] std::str::Utf8Error),
}

impl AssetLoader
for StyleSheetLoader
{
    type Asset = StyleSheetAsset;
    type Settings = ();
    type Error = StyleSheetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let content = std::str::from_utf8(&bytes)?;
        let stylesheet = StyleSheetAsset::parse(
            load_context.path().to_str().unwrap_or_default(),
            content
        );
        Ok(stylesheet)
    }

    fn extensions(
        &self
    ) -> &[&str] {
        &["css"]
    }
}
