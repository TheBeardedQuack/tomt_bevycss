use bevy::prelude::{
    Query, Entity, Parent, Node,
    With
};

pub type QueryEntityParent<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery
>;

pub type WorldQuery = (Entity, &'static Parent);
pub type ReadOnlyWorldQuery = With<Node>;
