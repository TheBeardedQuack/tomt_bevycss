use crate::prelude::StyleSheet;
use bevy::prelude::{
    Entity, Children, Parent,
    Query, Or, Changed,
};

#[cfg(feature = "monitor_changes")]
use bevy::ui::Interaction;

pub type QueryUiChanges<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (
    Entity,
    Option<&'static Parent>,
    Option<&'static Children>,
    Option<&'static StyleSheet>
);

#[cfg(not(feature = "monitor_changes"))]
pub type ReadOnlyWorldQuery = Changed<StyleSheet>;

#[cfg(feature = "monitor_changes")]
type ReadOnlyWorldQuery = Or<(Changed<StyleSheet>, Changed<Children>, Changed<Interaction>)>;
