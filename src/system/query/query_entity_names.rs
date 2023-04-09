use bevy::prelude::{
    Query, Entity, Name
};

pub type QueryEntityNames<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (Entity, &'static Name);
pub type ReadOnlyWorldQuery = ();
