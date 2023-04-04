use crate::selector::Selector;
use smallvec::SmallVec;
use bevy::{
    prelude::{
        Entity,
        Deref, DerefMut,
    },
    utils::HashMap,
};

/// Maps which entities was selected by a [`Selector`]
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct SelectedEntities(HashMap<Selector, SmallVec<[Entity; 8]>>);
