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

/// Apply a custom `text-content` which updates [`TextSection::value`](`TextSection`) of all sections on matched [`Text`] components
#[derive(Default)]
pub struct TextContentProperty;

impl Property
for TextContentProperty
{
    type Cache = String;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name(
        // no args
    ) -> &'static str {
        "text-content"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.string()
        {
            Some(content) => Ok(content),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        // TODO: Maybe change this so each line break is a new section
        for section in components.sections.iter_mut()
        {
            section.value = cache.clone()
        }
    }
}
