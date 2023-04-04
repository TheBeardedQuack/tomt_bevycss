use super::ComponentFilter;

use bevy::{
    prelude::Resource,
    utils::HashMap,
};

#[derive(Default, Resource)]
pub(crate) struct ComponentFilterRegistry(
    pub HashMap<
        &'static str,
        Box<dyn ComponentFilter + Send + Sync>
    >,
);
