use bevy::prelude::{Children, Entity, Node, Query, With};

pub type QueryEntityChildren<'w, 's> = Query<'w, 's, WorldQuery, ReadOnlyWorldQuery>;

pub type WorldQuery = (Entity, &'static Children);
pub type ReadOnlyWorldQuery = With<Node>;
