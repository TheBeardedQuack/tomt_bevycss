use bevy::{
    ecs::system::SystemState,
    prelude::{
        Component,
        Entity,
        Query,
        With, World,
    },
};

use crate::DynArray;

pub(crate) trait ComponentFilter
{
    fn filter(
        &mut self,
        world: &World
    ) -> DynArray<Entity>;
}

impl<'w, 's, T: Component> ComponentFilter
for SystemState<Query<'w, 's, Entity, With<T>>>
{
    fn filter(
        &mut self,
        world: &World
    ) -> DynArray<Entity> {
        self.get(world).iter().collect()
    }
}
