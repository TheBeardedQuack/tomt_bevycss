use super::MatchSelectorElement;

use bevy::prelude::{
    Component,
    Deref,
    Reflect, ReflectComponent
};
use std::borrow::Cow;

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

impl Class
{
    /// Creates a new [`Class`] with the given class names.
    ///
    /// Multiple class names can be used separated by spaces.
    pub fn new(
        class: impl Into<Cow<'static, str>>
    ) -> Self {
        Self(class.into())
    }

    /// Checks if any of this class names matches the given class name
    pub fn contains_class(
        &self,
        class: &str
    ) -> bool {
        self.0.split_ascii_whitespace()
            .any(|c| c == class)
    }

    pub fn is_empty(
        &self
    ) -> bool {
        self.0.is_empty()
    }

    pub fn add_class(
        &mut self,
        class: &str
    ) -> &mut Self {
        if !self.contains_class(class)
        {
            match self.is_empty()
            {
                true => self.0.to_mut().push_str(class),
                false => {
                    self.0.to_mut().push(' ');
                    self.0.to_mut().push_str(class);
                }
            }
        }

        self
    }

    pub fn remove_class(
        &mut self,
        class: &str
    ) -> &mut Self {
        if self.contains_class(class)
        {
            self.0 = self.0.split_ascii_whitespace()
                .filter(move |&c| c != class)
                .collect::<Vec<_>>()
                .join(" ")
                .into();
        }

        self
    }
}

impl MatchSelectorElement
for Class
{
    fn matches(
        &self,
        element: &str
    ) -> bool {
        self.contains_class(element)
    }
}
