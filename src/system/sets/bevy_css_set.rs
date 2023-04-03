use bevy::ecs::schedule::SystemSet;

/// System sets  used by `tomt_bevycss` systems
#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BevyCssSet {
    /// Prepares internal state before running apply systems.
    /// This system runs on [`CoreSet::PreUpdate`].
    Prepare,
    /// All [`Property`] implementation `systems` are run on this system set.
    /// Those stages runs on [`CoreSet::PreUpdate`] after [`BevyCssSet::Prepare`].
    Apply,
    /// Clears the internal state used by [`Property`] implementation `systems` set.
    /// This system runs on [`CoreSet::PostUpdate`].
    Cleanup,
}
