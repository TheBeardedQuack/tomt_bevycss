use crate::prelude::StyleSheetAsset;

use bevy::prelude::{
    Component,
    Handle,
    Reflect, ReflectComponent,
};

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
/// ```
///
#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StyleSheet
{
    sheet: Handle<StyleSheetAsset>,
}

impl StyleSheet
{
    /// Creates a new [`StyleSheet`] from the given asset.
    pub fn new(
        handle: Handle<StyleSheetAsset>
    ) -> Self {
        Self{
            sheet: handle
        }
    }

    /// Reapplies the style sheet on entity and all children.
    pub fn refresh(
        &mut self
    ) {
        // Just to trigger DerefMut
    }

    /// Internal [`StyleSheetAsset`] handle
    pub fn handle(
        &self
    ) -> &Handle<StyleSheetAsset> {
        &self.sheet
    }

    /// Change the internal [`StyleSheetAsset`] handle.
    /// This will automatically trigger the systems to reapply the style sheet.
    pub fn set(
        &mut self,
        handle: Handle<StyleSheetAsset>
    ) {
        self.sheet = handle;
    }
}

impl PartialEq
for StyleSheet
{
    fn eq(
        &self,
        other: &Self
    ) -> bool {
        self.sheet == other.sheet
    }
}
