use super::*;
use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};

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
                match values.rect()
                {
                    Some(val) => Ok(val),
                    None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
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

impl_style_rect!("margin", MarginProperty, margin);
impl_style_rect!("padding", PaddingProperty, padding);
impl_style_rect!("border", BorderProperty, border);

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
                match values.$parse_func()
                {
                    Some(val) => Ok(val),
                    None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
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

// Val properties
impl_style_single_value!("left", LeftProperty, Val, val, position.left);
impl_style_single_value!("right", RightProperty, Val, val, position.right);
impl_style_single_value!("top", TopProperty, Val, val, position.top);
impl_style_single_value!("bottom", BottomProperty, Val, val, position.bottom);

impl_style_single_value!("width", WidthProperty, Val, val, size.width);
impl_style_single_value!("height", HeightProperty, Val, val, size.height);

impl_style_single_value!("min-width", MinWidthProperty, Val, val, min_size.width);
impl_style_single_value!("min-height", MinHeightProperty, Val, val, min_size.height);

impl_style_single_value!("max-width", MaxWidthProperty, Val, val, max_size.width);
impl_style_single_value!("max-height", MaxHeightProperty, Val, val, max_size.height);

impl_style_single_value!("flex-basis", FlexBasisProperty, Val, val, max_size.height);

impl_style_single_value!("flex-grow", FlexGrowProperty, f32, f32, flex_grow);
impl_style_single_value!("flex-shrink", FlexShrinkProperty, f32, f32, flex_shrink);

impl_style_single_value!(
    "aspect-ratio",
    AspectRatioProperty,
    Option<f32>,
    option_f32,
    aspect_ratio
);

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

impl_style_enum!(
    Display, "display",
    DisplayProperty, display,
    "flex" => Flex,
    "none" => None
);

impl_style_enum!(
    PositionType, "position-type",
    PositionTypeProperty, position_type,
    "absolute" => Absolute,
    "relative" => Relative,
);

impl_style_enum!(
    Direction, "direction",
    DirectionProperty, direction,
    "inherit" => Inherit,
    "left-to-right" => LeftToRight,
    "right-to-left" => RightToLeft,
);

impl_style_enum!(
    FlexDirection, "flex-direction",
    FlexDirectionProperty, flex_direction,
    "row" => Row,
    "column" => Column,
    "row-reverse" => RowReverse,
    "column-reverse" => ColumnReverse,
);

impl_style_enum!(
    FlexWrap, "flex-wrap",
    FlexWrapProperty, flex_wrap,
    "no-wrap" => NoWrap,
    "wrap" => Wrap,
    "wrap-reverse" => WrapReverse,
);

impl_style_enum!(
    AlignItems, "align-items",
    AlignItemsProperty, align_items,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "baseline" => Baseline,
    "stretch" => Stretch,
);

impl_style_enum!(
    AlignSelf, "align-self",
    AlignSelfProperty, align_self,
    "auto" => Auto,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "baseline" => Baseline,
    "stretch" => Stretch,
);

impl_style_enum!(
    AlignContent, "align-content",
    AlignContentProperty, align_content,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "stretch" => Stretch,
    "space-between" => SpaceBetween,
    "space-around" => SpaceAround,
);

impl_style_enum!(
    JustifyContent, "justify-content",
    JustifyContentProperty, justify_content,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "space-between" => SpaceBetween,
    "space-around" => SpaceAround,
    "space-evenly" => SpaceEvenly,
);

impl_style_enum!(
    Overflow, "overflow",
    OverflowProperty, overflow,
    "visible" => Visible,
    "hidden" => Hidden,
);
