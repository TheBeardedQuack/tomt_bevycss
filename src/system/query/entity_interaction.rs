use bevy::{
    prelude::{
        Entity,
        Query,
    },
    ui::Interaction,
};

pub type QueryEntityInteraction<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (Entity, &'static Interaction);
pub type ReadOnlyWorldQuery = ();
