use bevy::ecs::schedule::SystemSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct BevyCssHotReload;
