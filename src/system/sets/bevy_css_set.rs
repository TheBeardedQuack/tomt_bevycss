use bevy::ecs::schedule::SystemSet;

/// System sets  used by `tomt_bevycss` systems
#[derive(Debug, Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub enum BevyCssSet {
    /// Prepares internal state before running apply systems.
    /// This system runs on [`bevy::prelude::CoreSet::PreUpdate`].
    Prepare,
    /// All [`crate::prelude::Property`] implementation `systems` are run on this system set.
    /// Those stages runs on [`bevy::prelude::CoreSet::PreUpdate`] after [`BevyCssSet::Prepare`].
    Apply,
    /// Clears the internal state used by [`crate::prelude::Property`] implementation `systems` set.
    /// This system runs on [`bevy::prelude::CoreSet::PostUpdate`].
    Cleanup,
}
