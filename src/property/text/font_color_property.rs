use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};
use bevy::{
    ecs::query::QueryItem,
    prelude::{
        AssetServer,
        Color,
        Commands,
        Node,
        Text,
        With,
    },
};

/// Applies the `color` property on [`TextStyle::color`](`TextStyle`) field of all sections on matched [`Text`] components.
#[derive(Default)]
pub struct FontColorProperty;

impl Property
for FontColorProperty
{
    type Cache = Color;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name(
        // no args
    ) -> &'static str {
        "color"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.color()
        {
            Some(color) => Ok(color),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        for section in components.sections.iter_mut()
        {
            section.style.color = *cache;
        }
    }
}
