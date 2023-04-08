use cssparser::Token;

/// A property value token which was parsed from a CSS rule.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PropertyToken {
    /// A value which was parsed percent value, like `100%` or `73.23%`.
    Percentage(f32),
    /// A value which was parsed dimension value, like `10px` or `35em.
    ///
    /// Currently there is no distinction between [`length-values`](https://developer.mozilla.org/en-US/docs/Web/CSS/length).
    Dimension(f32),
    /// A numeric float value, like `31.1` or `43`.
    Number(f32),
    /// A plain identifier, like `none` or `center`.
    Identifier(String),
    /// A identifier prefixed by a hash, like `#001122`.
    Hash(String),
    /// A quoted string, like `"some value"`.
    String(String),
}

impl<'i> TryFrom<Token<'i>> for PropertyToken {
    type Error = ();

    fn try_from(
        token: Token<'i>
    ) -> Result<Self, Self::Error> {
        match token {
            Token::Ident(val) => Ok(Self::Identifier(val.to_string())),
            Token::Hash(val) => Ok(Self::Hash(val.to_string())),
            Token::IDHash(val) => Ok(Self::Hash(val.to_string())),
            Token::QuotedString(val) => Ok(Self::String(val.to_string())),
            Token::Number { value, .. } => Ok(Self::Number(value)),
            Token::Percentage { unit_value, .. } => Ok(Self::Percentage(unit_value * 100.0)),
            Token::Dimension { value, .. } => Ok(Self::Dimension(value)),
            _ => Err(()),
        }
    }
}
