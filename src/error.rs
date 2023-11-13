use std::{
    error::Error,
    fmt::Display
};

/// Errors which can happens while parsing `css` into [`crate::selector::Selector`] or [`crate::Property`].
// TODO: Change this to Cow<'static, str>
#[derive(Debug)]
pub enum BevyCssError
{
    /// An unsupported selector was found on a style sheet rule.
    UnsupportedSelector,

    /// An unsupported property was found on a style sheet rule.
    UnsupportedProperty(String),

    /// An invalid property value was found on a style sheet rule.
    InvalidPropertyValue(String),

    /// An invalid selector was found on a style sheet rule.
    InvalidSelector,

    /// An unexpected token was found on a style sheet rule.
    UnexpectedToken(String),
}

impl Error
for BevyCssError
{
    // nothing to do
}

impl Display
for BevyCssError
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self
        {
            BevyCssError::UnsupportedSelector => write!(formatter, "Unsupported selector"),
            BevyCssError::UnsupportedProperty(prop) => write!(formatter, "Unsupported property: {}", prop),
            BevyCssError::InvalidPropertyValue(value) => write!(formatter, "Invalid property value: {}", value),
            BevyCssError::InvalidSelector => write!(formatter, "Invalid selector"),
            BevyCssError::UnexpectedToken(token) => write!(formatter, "Unexpected token: {}", token),
        }
    }
}
