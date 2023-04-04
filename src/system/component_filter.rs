use smallvec::SmallVec;
use bevy::{
    prelude::{
        Entity, Component, World, 
        Query, With,
    },
    ecs::system::SystemState,
};

pub(crate) trait ComponentFilter
{
    fn filter(
        &mut self,
        world: &World
    ) -> SmallVec<[Entity; 8]>;
}

impl<'w, 's, T: Component> ComponentFilter for SystemState<Query<'w, 's, Entity, With<T>>>
{
    fn filter(
        &mut self,
        world: &World
    ) -> SmallVec<[Entity; 8]> {
        self.get(world).iter().collect()
    }
}
