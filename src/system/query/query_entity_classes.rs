use crate::prelude::Class;
use bevy::prelude::{
    Query, Entity
};

pub type QueryEntityClasses<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (Entity, &'static Class);
pub type ReadOnlyWorldQuery = ();
