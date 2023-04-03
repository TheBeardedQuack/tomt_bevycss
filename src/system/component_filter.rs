use smallvec::SmallVec;
use bevy::prelude::{
    Entity, World,
};

pub(crate) trait ComponentFilter {
    fn filter(&mut self, world: &World) -> SmallVec<[Entity; 8]>;
}
