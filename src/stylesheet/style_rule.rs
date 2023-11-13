use crate::{property::PropertyValues, selector::Selector};

use bevy::utils::HashMap;
use std::fmt;

/// Represents a single rule inside a style sheet with a [`Selector`] which determines which entities
/// should be applied the [`PropertyValues`].
///
/// Note that this struct holds intermediate data, the final value is parsed by [`Property`](crate::Property) on
/// the first time it's [`system`](crate::Property::apply_system) is invoked.
#[derive(Debug, Clone)]
pub struct StyleRule {
    /// Selector used to match entities to apply properties.
    pub selector: Selector,
    /// Properties values to be applied on selected entities.
    pub properties: HashMap<String, PropertyValues>,
}

impl StyleRule {
    pub fn new(selector: Selector) -> Self {
        Self {
            selector,
            properties: Default::default(),
        }
    }
}

impl fmt::Display for StyleRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.selector)
    }
}
