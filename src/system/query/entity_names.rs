use bevy::prelude::{Entity, Name, Query};

pub type QueryEntityNames<'w, 's> = Query<'w, 's, WorldQuery, ReadOnlyWorldQuery>;

pub type WorldQuery = (Entity, &'static Name);
pub type ReadOnlyWorldQuery = ();
