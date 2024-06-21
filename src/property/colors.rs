use bevy::{
    log::error,
    prelude::Color,
};

use cssparser_color::Color as CssColor;

pub(crate) fn to_bevy_color(
    css_color: CssColor
) -> Option<Color> {
    match css_color
    {
        CssColor::Rgba(color) => Some(Color::rgba_u8(
            color.red,
            color.green,
            color.blue,
            (color.alpha * u8::MAX as f32) as u8
        )),
        CssColor::ColorFunction(func) => {
            use cssparser::color::PredefinedColorSpace;
            match func.color_space
            {
                PredefinedColorSpace::Srgb => Some(Color::rgba(
                    func.c1.unwrap_or_default(),
                    func.c2.unwrap_or_default(),
                    func.c3.unwrap_or_default(),
                    func.alpha.unwrap_or(1.0)
                )),
                _ => {
                    error!("Unssupported source color format for color function: {css_color:?}");
                    None
                },
            }
        },
        _ => {
            error!("Unssupported source color format: {css_color:?}");
            None
        },
    }
}

pub(super) fn parse_hex_color(
    hex_str: &str
) -> Option<Color> {
    match cssparser::color::parse_hash_color(hex_str.as_bytes())
    {
        Ok((r, g, b, a)) => Some(
            Color::rgba_u8(r, g, b, (a * u8::MAX as f32) as u8)
        ),
        Err(_) => {
            error!("Failed to parse hex color: `{hex_str}`");
            None
        },
    }
}

// Source: https://developer.mozilla.org/en-US/docs/Web/CSS/named-color

/// Parses a named color, like "silver" or "azure" into a [`Color`]
///
/// Accepts any [valid CSS named-colors](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color).
pub(super) fn parse_named_color(
    name: &str
) -> Option<Color> {
    match cssparser_color::parse_color_keyword(name)
    {
        Ok(color) => to_bevy_color(color),
        Err(_) => {
            error!("Failed to parse named color: `{name}`");
            None
        },
    }
}
