
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
