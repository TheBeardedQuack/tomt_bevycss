use bevy::prelude::{Entity, Node, Parent, Query, With};

pub type QueryEntityParent<'w, 's> = Query<'w, 's, WorldQuery, ReadOnlyWorldQuery>;

pub type WorldQuery = (Entity, &'static Parent);
pub type ReadOnlyWorldQuery = With<Node>;
