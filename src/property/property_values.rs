use super::{
    colors,
    PropertyToken,
};

use bevy::{
    prelude::{
        Color,
        Deref
    },
    ui::{
        OverflowAxis,
        UiRect,
        Val,
    },
};
use bevy::prelude::BorderRadius;
use smallvec::SmallVec;

/// A list of [`PropertyToken`] which was parsed from a single property.
#[derive(Clone, Debug, Default)]
#[derive(Deref)]
pub struct PropertyValues(
    pub(crate) SmallVec<[PropertyToken; 8]>
);

impl PropertyValues
{
    /// Tries to parses the current values as a single [`String`].
    pub fn string(
        &self
    ) -> Option<String> {
        self.0.iter()
        .find_map(|token| match token
        {
            PropertyToken::String(id) => match id.is_empty()
            {
                true => None,
                false => Some(id.clone()),
            },
            _ => None,
        })
    }

    /// Tries to parses the current values as a single [`Color`].
    ///
    /// Currently only [named colors](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color)
    /// and [hex-colors](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) are supported.
    pub fn color(
        &self
    ) -> Option<Color> {
        if self.0.len() == 1
        {
            match &self.0[0]
            {
                PropertyToken::Identifier(name) => colors::parse_named_color(name.as_str()),
                PropertyToken::Hash(hash) => colors::parse_hex_color(hash.as_str()),
                _ => None,
            }
        }
        else
        {
            // TODO: Implement color function like rgba(255, 255, 255, 255)
            // https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
            None
        }
    }

    /// Tries to parse the current value as a single [`OverflowAxis`].
    pub fn overflow(
        &self
    ) -> Option<OverflowAxis> {
        if self.0.len() == 1
        {
            match &self.0[0]
            {
                PropertyToken::Identifier(overflow) => match overflow.as_ref()
                {
                    "visible" => Some(OverflowAxis::Visible),
                    "hidden" | "clip" => Some(OverflowAxis::Clip),
                    _ => None,
                },
                _ => None,
            }
        }
        else
        {
            None
        }
    }

    /// Tries to parses the current values as a single identifier.
    pub fn identifier(
        &self
    ) -> Option<&str> {
        self.0.iter()
            .find_map(|token| match token
            {
                PropertyToken::Identifier(id) => match id.is_empty()
                {
                    true => None,
                    false => Some(id.as_str()),
                }
                _ => None,
            })
    }

    /// Tries to parses the current values as a single [`Val`].
    ///
    /// Only [`Percentage`](PropertyToken::Percentage) and [`Dimension`](PropertyToken::Dimension`) are considered valid values,
    /// where former is converted to [`Val::Percent`] and latter is converted to [`Val::Px`].
    pub fn val(
        &self
    ) -> Option<Val> {
        self.0.iter()
            .find_map(|token| match token
            {
                PropertyToken::Percentage(val) => Some(Val::Percent(*val)),
                PropertyToken::Dimension(val) => Some(Val::Px(*val)),
                PropertyToken::Identifier(val) if val == "auto" => Some(Val::Auto),
                _ => None,
            })
    }

    /// Tries to parses the current values as a single [`f32`].
    ///
    /// Only [`Percentage`](PropertyToken::Percentage), [`Dimension`](PropertyToken::Dimension`) and [`Number`](PropertyToken::Number`)
    /// are considered valid values.
    pub fn f32(
        &self
    ) -> Option<f32> {
        self.0.iter()
            .find_map(|token| match token
            {
                PropertyToken::Percentage(val)
                | PropertyToken::Dimension(val)
                | PropertyToken::Number(val) => Some(*val),
                _ => None,
            })
    }

    /// Tries to parses the current values as a single [`Option<f32>`].
    ///
    /// This function is useful for properties where either a numeric value or a `none` value is expected.
    ///
    /// If a [`Option::None`] is returned, it means some invalid value was found.
    ///
    /// If there is a [`Percentage`](PropertyToken::Percentage), [`Dimension`](PropertyToken::Dimension`) or [`Number`](PropertyToken::Number`) token,
    /// a [`Option::Some`] with parsed [`Option<f32>`] is returned.
    /// If there is a identifier with a `none` value, then [`Option::Some`] with [`None`] is returned.
    pub fn option_f32(
        &self
    ) -> Option<Option<f32>> {
        self.0.iter()
            .find_map(|token| match token
            {
                PropertyToken::Percentage(val)
                | PropertyToken::Dimension(val)
                | PropertyToken::Number(val) => Some(Some(*val)),

                PropertyToken::Identifier(ident) => match ident.as_str()
                {
                    "none" => Some(None),
                    _ => None,
                },

                _ => None,
            })
    }

    /// Tries to parses the current values as a single [`Option<UiRect>`].
    ///
    /// Optional values are handled by this function, so if only one value is present it is used as `top`, `right`, `bottom` and `left`,
    /// otherwise values are applied in the following order: `top`, `right`, `bottom` and `left`.
    ///
    /// Note that it is not possible to create a [`UiRect`] with only `top` value, since it'll be understood to replicated it on all fields.
    pub fn rect(
        &self
    ) -> Option<UiRect> {
        if self.0.len() == 1
        {
            self.val().map(UiRect::all)
        }
        else
        {
            self.0.iter()
                .fold((None, 0), |(rect, idx), token|
                {
                    let val = match token
                    {
                        PropertyToken::Percentage(val) => Val::Percent(*val),
                        PropertyToken::Dimension(val) => Val::Px(*val),
                        PropertyToken::Identifier(val) if val == "auto" => Val::Auto,
                        _ => return (rect, idx),
                    };
                    let mut rect: UiRect = rect.unwrap_or_default();

                    match idx
                    {
                        0 => rect.top = val,
                        1 => rect.right = val,
                        2 => rect.bottom = val,
                        3 => rect.left = val,
                        _ => (),
                    }
                    (Some(rect), idx + 1)
                }).0
        }
    }

    /// Tries to parses the current values as a single [`Option<BorderRadius>`].
    pub fn border_radius(
        &self
    ) -> Option<BorderRadius> {
        if self.0.len() == 1
        {
            self.val().map(BorderRadius::all)
        }
        else
        {
            self.0.iter()
                .fold((None, 0), |(border_radius, idx), token|
                {
                        let val = match token
                        {
                            PropertyToken::Percentage(val) => Val::Percent(*val),
                            PropertyToken::Dimension(val) => Val::Px(*val),
                            PropertyToken::Number(num) if *num == 0.0 => Val::Px(0.0),
                            PropertyToken::Identifier(val) if val == "auto" => Val::Auto,
                            _ => return (border_radius, idx),
                        };
                        let mut border_radius: BorderRadius = border_radius.unwrap_or_default();

                        match idx
                        {
                            0 => border_radius = BorderRadius::all(val),
                            1 => {
                                border_radius.top_right = val;
                                border_radius.bottom_left = val;
                            }
                            2 => border_radius.bottom_right = val,
                            3 => border_radius.bottom_left = val,
                            _ => (),
                        }
                        (Some(border_radius), idx + 1)
                }).0
        }
    }
}
