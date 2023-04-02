use super::MatchSelectorElement;
use std::borrow::Cow;
use bevy::prelude::{
    Component,
    Deref,
    Reflect, ReflectComponent,
};

/// Sets the entities class to be matched by selectors in on`css`.
///
/// The behavior mimics CSS so a single class name can given or a list separated by spaces.
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use tomt_bevycss::prelude::*;
/// fn system(mut commands: Commands) {
///     // This entity can be selected by either ".yellow-button", ".enabled"
///     // or even ".yellow-button.enabled"
///     commands.spawn(Class::new("yellow-button enabled"));
/// }
/// ```
#[derive(Debug, Reflect, Component, Default, Clone, Deref)]
#[reflect(Component)]
pub struct Class(Cow<'static, str>);

impl Class {
    /// Creates a new [`Class`] with the given class names.
    ///
    /// Multiple class names can be used separated by spaces.
    pub fn new(class: impl Into<Cow<'static, str>>) -> Self {
        Self(class.into())
    }

    /// Checks if any of this class names matches the given class name
    fn matches(&self, class: &str) -> bool {
        self.0.split_ascii_whitespace().any(|c| c == class)
    }
}

impl MatchSelectorElement for Class {
    fn matches(&self, element: &str) -> bool {
        self.matches(element)
    }
}
