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

/// Applies the `font` property on [`TextStyle::font`](`TextStyle`) property of all sections on matched [`Text`] components.
#[derive(Default)]
pub(crate) struct FontProperty;

impl Property for FontProperty
{
    type Cache = String;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name()
    -> &'static str
    {
        "font"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>
    {
        if let Some(path) = values.string() {
            Ok(path)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components
            .sections
            .iter_mut()
            .for_each(|section| section.style.font = asset_server.load(cache));
    }
}
