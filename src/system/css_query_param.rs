use super::query;

#[cfg(feature = "pseudo_prop")]
use crate::prelude::PseudoProp;
use crate::prelude::StyleSheetAsset;

use bevy::{
    prelude::*,
    ecs::system::SystemParam,
};

#[derive(SystemParam)]
pub(crate) struct CssQueryParam<'w, 's>
{
    pub assets: StyleSheetResource<'w>,
    pub ui_nodes: query::QueryUiNodes<'w, 's>,
    pub ui_changes: query::QueryUiChanges<'w, 's>,
    pub names: query::QueryEntityNames<'w, 's>,
    pub classes: query::QueryEntityClasses<'w, 's>,
    pub children: query::QueryEntityChildren<'w, 's>,
    
    #[cfg(feature = "pseudo_class")]
    pub pseudo_classes: PseudoClassParam<'w, 's>,
}

#[derive(Deref, SystemParam)]
pub(crate) struct StyleSheetResource<'w>(
    Res<'w, Assets<StyleSheetAsset>>
);

#[cfg(feature = "pseudo_class")]
#[derive(SystemParam)]
pub(crate) struct PseudoClassParam<'w, 's>
{
    pub interaction: query::QueryEntityInteraction<'w, 's>,
    pub _children: query::QueryEntityChildren<'w, 's>,
}
