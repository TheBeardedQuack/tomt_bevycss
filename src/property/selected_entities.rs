use crate::selector::Selector;
use bevy::{
    prelude::{Deref, DerefMut, Entity},
    utils::HashMap,
};
use smallvec::SmallVec;

/// Maps which entities was selected by a [`Selector`]
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct SelectedEntities(HashMap<Selector, SmallVec<[Entity; 8]>>);
