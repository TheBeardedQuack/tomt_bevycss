use bevy::{
    log::error,
    prelude::Color,
};

pub(crate) fn to_bevy_color(
    css_color: cssparser::Color
) -> Option<Color> {
    match css_color
    {
        cssparser::Color::Rgba(color) => Some(Color::rgba_u8(
            color.red.unwrap_or_default(),
            color.green.unwrap_or_default(),
            color.blue.unwrap_or_default(),
            (color.alpha.unwrap_or_default() * u8::MAX as f32) as u8
        )),
        _ => {
            error!("Unssupported source color format: {css_color:?}");
            None
        },
    }
}

pub(super) fn parse_hex_color(
    hex_str: &str
) -> Option<Color> {
    match cssparser::parse_hash_color(hex_str.as_bytes())
    {
        Ok(color) => to_bevy_color(color),
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
    match cssparser::parse_color_keyword(name)
    {
        Ok(color) => to_bevy_color(color),
        Err(_) => {
            error!("Failed to parse named color: `{name}`");
            None
        },
    }
}
