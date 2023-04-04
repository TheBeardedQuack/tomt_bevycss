use crate::{
    prelude::BevyCssError,
    property::{
        Property,
        PropertyValues,
    },
};
use bevy::{
    ecs::query::QueryItem,
    prelude::{
        AssetServer,
        Commands, 
        Text,
        Node,
        With,
    },
};

/// Apply a custom `text-content` which updates [`TextSection::value`](`TextSection`) of all sections on matched [`Text`] components
#[derive(Default)]
pub struct TextContentProperty;

impl Property for TextContentProperty
{
    type Cache = String;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name()
    -> &'static str
    {
        "text-content"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>
    {
        if let Some(content) = values.string() {
            Ok(content)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components
            .sections
            .iter_mut()
            // TODO: Maybe change this so each line break is a new section
            .for_each(|section| section.value = cache.clone());
    }
}
