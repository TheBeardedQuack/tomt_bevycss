use crate::{DynArray, selector::Selector};

use bevy::{
    prelude::{
        Deref, DerefMut,
        Entity,
    },
    utils::HashMap,
};

/// Maps which entities was selected by a [`Selector`]
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct SelectedEntities(
    HashMap<
        Selector,
        DynArray<Entity>
    >
);
