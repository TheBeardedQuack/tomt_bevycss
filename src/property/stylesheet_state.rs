use super::SelectedEntities;
use crate::stylesheet::StyleSheetAsset;
use bevy::{
    prelude::{
        Handle, Resource,
        Deref, DerefMut,
    },
    utils::HashMap,
};

/// Maps sheets for each [`StyleSheetAsset`].
#[derive(Debug, Clone, Default, Deref, DerefMut, Resource)]
pub struct StyleSheetState(
    HashMap<
        Handle<StyleSheetAsset>,
        SelectedEntities
    >
);
