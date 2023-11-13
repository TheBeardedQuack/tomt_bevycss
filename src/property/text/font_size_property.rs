use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};
use bevy::{
    ecs::query::QueryItem,
    prelude::{
        AssetServer,
        Commands,
        Node,
        Text,
        With,
    },
};

/// Applies the `font-size` property on [`TextStyle::font_size`](`TextStyle`) property of all sections on matched [`Text`] components.
#[derive(Default)]
pub struct FontSizeProperty;

impl Property
for FontSizeProperty
{
    type Cache = f32;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name(
        // no args
    ) -> &'static str {
        "font-size"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.f32()
        {
            Some(size) => Ok(size),
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
            section.style.font_size = *cache
        }
    }
}
