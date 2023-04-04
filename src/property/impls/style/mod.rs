#[macro_use]
mod macros;

use super::*;
use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};

// Rect type property fields
impl_style_rect!("margin", MarginProperty, margin);
impl_style_rect!("padding", PaddingProperty, padding);
impl_style_rect!("border", BorderProperty, border);

// Val (number) type property fields
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

// f32 (number) type property fields
impl_style_single_value!("flex-grow", FlexGrowProperty, f32, f32, flex_grow);
impl_style_single_value!("flex-shrink", FlexShrinkProperty, f32, f32, flex_shrink);

impl_style_single_value!("aspect-ratio", AspectRatioProperty, Option<f32>, option_f32, aspect_ratio);

impl_style_enum!(
    Display,            // Bevy enum
    "display",          // CSS property name
    DisplayProperty,    // Library structure to map
    display,            // Property to access on bevy::ui::Style

    "flex" => Flex,     // Text-to-Bevy enum mappings
    "none" => None
);

impl_style_enum!(
    PositionType, "position-type", PositionTypeProperty, position_type,
    "absolute" => Absolute,
    "relative" => Relative,
);

impl_style_enum!(
    Direction, "direction", DirectionProperty, direction,
    "inherit" => Inherit,
    "left-to-right" => LeftToRight,
    "right-to-left" => RightToLeft,
);

impl_style_enum!(
    FlexDirection, "flex-direction", FlexDirectionProperty, flex_direction,
    "row" => Row,
    "column" => Column,
    "row-reverse" => RowReverse,
    "column-reverse" => ColumnReverse,
);

impl_style_enum!(
    FlexWrap, "flex-wrap", FlexWrapProperty, flex_wrap,
    "no-wrap" => NoWrap,
    "wrap" => Wrap,
    "wrap-reverse" => WrapReverse,
);

impl_style_enum!(
    AlignItems, "align-items", AlignItemsProperty, align_items,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "baseline" => Baseline,
    "stretch" => Stretch,
);

impl_style_enum!(
    AlignSelf, "align-self", AlignSelfProperty, align_self,
    "auto" => Auto,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "baseline" => Baseline,
    "stretch" => Stretch,
);

impl_style_enum!(
    AlignContent, "align-content", AlignContentProperty, align_content,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "stretch" => Stretch,
    "space-between" => SpaceBetween,
    "space-around" => SpaceAround,
);

impl_style_enum!(
    JustifyContent, "justify-content", JustifyContentProperty, justify_content,
    "flex-start" => FlexStart,
    "flex-end" => FlexEnd,
    "center" => Center,
    "space-between" => SpaceBetween,
    "space-around" => SpaceAround,
    "space-evenly" => SpaceEvenly,
);

impl_style_enum!(
    Overflow, "overflow", OverflowProperty, overflow,
    "visible" => Visible,
    "hidden" => Hidden,
);
