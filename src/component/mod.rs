mod class;
pub use class::*;

mod style_sheet;
pub use style_sheet::*;

use bevy::prelude::Name;

/// Convenience trait which matches matches a component against a named element selector.
pub(crate) trait MatchSelectorElement {
    fn matches(&self, element: &str) -> bool;
}

impl MatchSelectorElement for Name {
    fn matches(&self, element: &str) -> bool {
        self.as_str() == element
    }
}
