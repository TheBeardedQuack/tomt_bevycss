use crate::{
    error::BevyCssError,
    property::{Property, PropertyValues},
};
use bevy::{
    ecs::query::QueryItem,
    prelude::*,
};

/// Applies the `background-color` property on [`BackgroundColor`] component of matched entities.
#[derive(Default)]
pub struct BackgroundColorProperty;

impl Property for BackgroundColorProperty {
    type Cache = Color;
    type Components = Entity;
    type Filters = With<BackgroundColor>;

    fn name() -> &'static str {
        "background-color"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        if let Some(color) = values.color() {
            Ok(color)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        commands: &mut Commands,
    ) {
        commands.entity(components).insert(BackgroundColor(*cache));
    }
}
