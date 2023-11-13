use crate::prelude::StyleSheet;
use bevy::prelude::{
    Children,
    Entity,
    Parent,
    Query,
};

pub type QueryUiNodes<'w, 's> = Query<
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

pub type ReadOnlyWorldQuery = ();
