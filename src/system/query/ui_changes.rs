use crate::prelude::StyleSheet;
use bevy::prelude::{
    Query, Children, Or, Changed,
};

#[cfg(feature = "monitor_changes")]
use bevy::ui::Interaction;

pub type QueryUiChanges<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = super::ui_nodes::WorldQuery;

#[cfg(not(feature = "monitor_changes"))]
pub type ReadOnlyWorldQuery = Changed<StyleSheet>;

#[cfg(feature = "monitor_changes")]
type ReadOnlyWorldQuery = Or<(Changed<StyleSheet>, Changed<Children>, Changed<Interaction>)>;
