use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, Hash, Eq, PartialEq)]
#[system_set(base)]
pub struct BevyCssHotReload;
