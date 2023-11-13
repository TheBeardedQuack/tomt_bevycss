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
        Text, TextAlignment,
        With,
    },
};

/// Applies the `text-align` property on [`Text::horizontal`](`TextAlignment`) components.
#[derive(Default)]
pub struct TextAlignProperty;

impl Property
for TextAlignProperty
{
    // Using Option since Cache must impl Default, which  doesn't
    type Cache = Option<TextAlignment>;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name(
        // no args
    ) -> &'static str {
        "text-align"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.identifier()
        {
            Some("left") => Ok(Some(TextAlignment::Left)),
            Some("center") => Ok(Some(TextAlignment::Center)),
            Some("right") => Ok(Some(TextAlignment::Right)),
            _ => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components.alignment = cache.expect("Should always have a inner value");
    }
}
