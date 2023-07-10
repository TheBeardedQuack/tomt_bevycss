use bevy::ecs::schedule::{
    ScheduleLabel,
    SystemSet,
};

#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(ScheduleLabel, SystemSet)]
pub struct BevyCssHotReload;
