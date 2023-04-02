mod class;
pub use class::*;

use bevy::prelude::{
    Component,
    Handle,
    Name,
    Reflect,
    ReflectComponent
};

use crate::StyleSheetAsset;

/// Applies a [`StyleSheetAsset`] on the entity which has this component.
///
/// Note that style rules are applied only once when the component is added, or if the asset is changed
/// and [hot_reloading](https://github.com/bevyengine/bevy/blob/main/examples/asset/hot_asset_reloading.rs) is enabled.
/// If you want to reapply the stylesheet, like when new children was added, use [`StyleSheet::refresh`].
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// use tomt_bevycss::prelude::*;
///
/// fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
///     commands.spawn(StyleSheet::new(asset_server.load("sheets/fancy.css")));
/// }
///
/// ```
///
#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct StyleSheet {
    sheet: Handle<StyleSheetAsset>,
}

impl StyleSheet {
    /// Creates a new [`StyleSheet`] from the given asset.
    pub fn new(handle: Handle<StyleSheetAsset>) -> Self {
        Self { sheet: handle }
    }

    /// Reapplies the style sheet on entity and all children.
    pub fn refresh(&mut self) {
        // Just to trigger DerefMut
    }

    /// Internal [`StyleSheetAsset`] handle
    pub fn handle(&self) -> &Handle<StyleSheetAsset> {
        &self.sheet
    }

    /// Change the internal [`StyleSheetAsset`] handle.
    /// This will automatically trigger the systems to reapply the style sheet.
    pub fn set(&mut self, handle: Handle<StyleSheetAsset>) {
        self.sheet = handle;
    }
}

impl PartialEq for StyleSheet {
    fn eq(&self, other: &Self) -> bool {
        self.sheet == other.sheet
    }
}

/// Convenience trait which matches matches a component against a named element selector.
pub(crate) trait MatchSelectorElement {
    fn matches(&self, element: &str) -> bool;
}

impl MatchSelectorElement for Name {
    fn matches(&self, element: &str) -> bool {
        self.as_str() == element
    }
}
