use super::StyleSheetAsset;
use bevy::asset::{
    AssetLoader,
    LoadedAsset,
};

#[derive(Default)]
pub(crate) struct StyleSheetLoader;

impl AssetLoader for StyleSheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let content = std::str::from_utf8(bytes)?;
            let stylesheet = StyleSheetAsset::parse(
                load_context.path().to_str().unwrap_or_default(),
                content
            );
            load_context.set_default_asset(LoadedAsset::new(stylesheet));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["css"]
    }
}
