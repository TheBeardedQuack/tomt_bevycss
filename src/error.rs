use std::{error::Error, fmt::Display};

/// Errors which can happens while parsing `css` into [`crate::selector::Selector`] or [`crate::Property`].
// TODO: Change this to Cow<'static, str>
#[derive(Debug)]
pub enum BevyCssError {
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

impl Error for BevyCssError {}

impl Display for BevyCssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BevyCssError::UnsupportedSelector => {
                write!(f, "Unsupported selector")
            }
            BevyCssError::UnsupportedProperty(p) => {
                write!(f, "Unsupported property: {}", p)
            },
            BevyCssError::InvalidPropertyValue(p) => {
                write!(f, "Invalid property value: {}", p)
            },
            BevyCssError::InvalidSelector => {
                write!(f, "Invalid selector")
            }
            BevyCssError::UnexpectedToken(t) => {
                write!(f, "Unexpected token: {}", t)
            }
        }
    }
}
