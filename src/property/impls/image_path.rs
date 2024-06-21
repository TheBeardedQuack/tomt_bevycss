use crate::{
    error::BevyCssError,
    property::{
        Property,
        PropertyValues,
    },
};
use bevy::{
    ecs::query::QueryItem,
    prelude::*,
};

/// Applies the `image-path` property on [`UiImage`] component of matched entities.
#[derive(Default)]
pub struct ImagePathProperty;

impl Property
for ImagePathProperty
{
    type Cache = String;
    type Components = &'static mut UiImage;
    type Filters = With<Node>;

    fn name() -> &'static str {
        "image-path"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.string() {
            Some(path) => Ok(path),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }
    
    fn apply(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components.texture = asset_server.load(cache);
    }
}
