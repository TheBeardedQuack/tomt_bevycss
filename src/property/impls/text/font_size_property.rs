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

/// Applies the `font-size` property on [`TextStyle::font_size`](`TextStyle`) property of all sections on matched [`Text`] components.
#[derive(Default)]
pub(crate) struct FontSizeProperty;

impl Property for FontSizeProperty
{
    type Cache = f32;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name()
    -> &'static str
    {
        "font-size"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>
    {
        if let Some(size) = values.f32() {
            Ok(size)
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
            .for_each(|section| section.style.font_size = *cache);
    }
}
