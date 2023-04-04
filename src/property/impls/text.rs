use super::*;

/// Applies the `color` property on [`TextStyle::color`](`TextStyle`) field of all sections on matched [`Text`] components.
#[derive(Default)]
pub(crate) struct FontColorProperty;

impl Property for FontColorProperty
{
    type Cache = Color;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name()
    -> &'static str
    {
        "color"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>
    {
        if let Some(color) = values.color() {
            Ok(color)
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
            .for_each(|section| section.style.color = *cache);
    }
}

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

/// Applies the `text-align` property on [`Text::horizontal`](`TextAlignment`) components.
#[derive(Default)]
pub(crate) struct TextAlignProperty;

impl Property for TextAlignProperty
{
    // Using Option since Cache must impl Default, which  doesn't
    type Cache = Option<TextAlignment>;
    type Components = &'static mut Text;
    type Filters = With<Node>;

    fn name()
    -> &'static str
    {
        "text-align"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>
    {
        if let Some(ident) = values.identifier() {
            match ident {
                "left" => return Ok(Some(TextAlignment::Left)),
                "center" => return Ok(Some(TextAlignment::Center)),
                "right" => return Ok(Some(TextAlignment::Right)),
                _ => (),
            }
        }
        Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
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

/// Apply a custom `text-content` which updates [`TextSection::value`](`TextSection`) of all sections on matched [`Text`] components
#[derive(Default)]
pub(crate) struct TextContentProperty;

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
