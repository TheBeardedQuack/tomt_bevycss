use crate::prelude::Class;
use bevy::prelude::{
    Entity,
    Query,
};

pub type QueryEntityClasses<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (Entity, &'static Class);
pub type ReadOnlyWorldQuery = ();
