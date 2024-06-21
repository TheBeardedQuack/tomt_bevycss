use crate::{
    error::BevyCssError,
    property::{
        Property,
        PropertyValues,
    },
    Color,
};
use bevy::{
    ecs::query::QueryItem,
    prelude::*,
};

/// Applies the `border-color` property on [`BorderColor`] component of matched entities.
#[derive(Default)]
pub struct BorderColorProperty;

impl Property
for BorderColorProperty
{
    type Cache = Color;
    type Components = Entity;
    type Filters = With<BorderColor>;

    fn name() -> &'static str {
        "border-color"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.color() {
            Some(color) => Ok(color),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }
    
    fn apply(
        cache: &Self::Cache,
        components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        commands: &mut Commands,
    ) {
        commands.entity(components)
            .insert(BorderColor(*cache));
    }
}
