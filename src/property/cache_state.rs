/// Internal cache state. Used by [`CachedProperties`] to avoid parsing properties of the same rule on same sheet.
#[derive(Default, Debug, Clone)]
pub enum CacheState<T> {
    /// No parse was performed yet
    #[default]
    None,
    /// Parse was performed and yielded a valid value.
    Ok(T),
    /// Parse was performed but returned an error.
    Error,
}
