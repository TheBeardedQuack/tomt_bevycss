/// Implements a new property for [`Style`] component which expects a rect value.
macro_rules! impl_style_rect
{
    ($name:expr, $struct:ident, $style_prop:ident$(.$style_field:ident)*) => {
        #[doc = "Applies the `"]
        #[doc = $name]
        #[doc = "` property on [Style::"]
        #[doc = stringify!($style_prop)]
        $(#[doc = concat!("::",stringify!($style_field))])*
        #[doc = "](`Style`) field of all sections on matched [`Style`] components."]
        #[derive(Default)]
        pub(crate) struct $struct;

        impl Property for $struct
        {
            type Cache = UiRect;
            type Components = &'static mut Style;
            type Filters = With<Node>;

            fn name()
            -> &'static str {
                $name
            }

            fn parse<'a>(
                values: &PropertyValues
            ) -> Result<Self::Cache, BevyCssError> {
                if let Some(val) = values.rect()
                {
                    Ok(val)
                }
                else
                {
                    Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
                }
            }

            fn apply<'w>(
                cache: &Self::Cache,
                mut components: QueryItem<Self::Components>,
                _asset_server: &AssetServer,
                _commands: &mut Commands,
            ) {
                components.$style_prop$(.$style_field)? = *cache;
            }
        }
    };
}

/// Implements a new property for [`Style`] component which expects a single value.
macro_rules! impl_style_single_value
{
    ($name:expr, $struct:ident, $cache:ty, $parse_func:ident, $style_prop:ident$(.$style_field:ident)*) => {
        #[doc = "Applies the `"]
        #[doc = $name]
        #[doc = "` property on [Style::"]
        #[doc = stringify!($style_prop)]
        $(#[doc = concat!("::",stringify!($style_field))])*
        #[doc = "](`Style`) field of all sections on matched [`Style`] components."]
        #[derive(Default)]
        pub(crate) struct $struct;

        impl Property for $struct
        {
            type Cache = $cache;
            type Components = &'static mut Style;
            type Filters = With<Node>;

            fn name()
            -> &'static str
            {
                $name
            }

            fn parse<'a>(
                values: &PropertyValues
            ) -> Result<Self::Cache, BevyCssError>
            {
                if let Some(val) = values.$parse_func()
                {
                    Ok(val)
                }
                else
                {
                    Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
                }
            }

            fn apply<'w>(
                cache: &Self::Cache,
                mut components: QueryItem<Self::Components>,
                _asset_server: &AssetServer,
                _commands: &mut Commands,
            ) {
                components.$style_prop$(.$style_field)? = *cache;
            }
        }
    };
}

/// Implements a new property for [`Style`] component which expects an enum.
macro_rules! impl_style_enum
{
    ($cache:ty, $name:expr, $struct:ident, $style_prop:ident, $($prop:expr => $variant:expr),+$(,)?) => {
        #[doc = "Applies the `"]
        #[doc = $name]
        #[doc = "` property on [Style::"]
        #[doc = stringify!($style_prop)]
        #[doc = "]("]
        #[doc = concat!("`", stringify!($cache), "`")]
        #[doc = ") field of all sections on matched [`Style`] components."]
        #[derive(Default)]
        pub(crate) struct $struct;

        impl Property for $struct
        {
            type Cache = $cache;
            type Components = &'static mut Style;
            type Filters = With<Node>;

            fn name()
            -> &'static str
            {
                $name
            }

            fn parse<'a>(
                values: &PropertyValues
            ) -> Result<Self::Cache, BevyCssError>
            {
                if let Some(identifier) = values.identifier()
                {
                    use $cache::*;
                    // Chain if-let when `cargofmt` supports it
                    // https://github.com/rust-lang/rustfmt/pull/5203
                    match identifier
                    {
                        $($prop => return Ok($variant)),+,
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
                components.$style_prop = *cache;
            }
        }
    };
}
