use bevy::{
    ecs::system::SystemState,
    prelude::{
        Component,
        Entity,
        Query,
        With, World,
    },
};
use smallvec::SmallVec;

pub(crate) trait ComponentFilter
{
    fn filter(
        &mut self,
        world: &World
    ) -> SmallVec<[Entity; 8]>;
}

impl<'w, 's, T: Component> ComponentFilter
for SystemState<Query<'w, 's, Entity, With<T>>>
{
    fn filter(
        &mut self,
        world: &World
    ) -> SmallVec<[Entity; 8]> {
        self.get(world).iter().collect()
    }
}
