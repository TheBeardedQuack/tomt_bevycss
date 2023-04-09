use crate::prelude::{
    Class,
    StyleSheet,
    StyleSheetAsset,
};

#[cfg(feature = "pseudo_prop")]
use crate::prelude::PseudoProp;

use bevy::{
    prelude::*,
    ecs::{system::SystemParam, query::{WorldQuery, ReadOnlyWorldQuery, ROQueryItem, QueryIter}},
};

#[derive(SystemParam)]
pub(crate) struct CssQueryParam<'w, 's>
{
    pub assets: StyleSheetResource<'w>,
    pub nodes: Query<
        'w, 's,
        (Entity, Option<&'static Children>, &'static StyleSheet),
    >,
    pub ui_changes: QueryUiChanges<'w, 's>,
    pub names: QueryEntityNames<'w, 's>,
    pub classes: QueryEntityClasses<'w, 's>,
    pub children: QueryChildren<'w, 's>,
    
    #[cfg(feature = "pseudo_class")]
    pub pseudo_classes: PseudoClassParam<'w, 's>,
}

pub(crate) type QueryUiChangesItem = (
    Entity,
    Option<&'static Parent>,
    Option<&'static Children>,
    Option<&'static StyleSheet>
);

#[derive(Deref, SystemParam)]
pub(crate) struct QueryUiChanges<'w, 's>(
    Query<
        'w, 's,
        QueryUiChangesItem,
        FilterUiChanges
    >
);

impl<
    'w, 's,
> IntoIterator for &'w QueryUiChanges<'_, 's> {
    type Item = ROQueryItem<'w, QueryUiChangesItem>;
    type IntoIter = QueryIter<'w, 's, QueryUiChangesItem, FilterUiChanges>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Deref, SystemParam)]
pub(crate) struct StyleSheetResource<'w>(
    Res<'w, Assets<StyleSheetAsset>>
);

#[cfg(not(feature = "pseudo_class"))]
#[derive(Deref)]
pub(crate) type FilterUiChanges = Changed<StyleSheet>;

#[cfg(feature = "pseudo_class")]
pub(crate) type FilterUiChanges = Or<(
    Changed<StyleSheet>,
    Changed<Children>,
    Changed<Interaction>
)>;

#[derive(Deref, SystemParam)]
pub(crate) struct QueryEntityNames<'w, 's>(
    Query<
        'w, 's,
        (Entity, &'static Name)
    >
);

#[derive(Deref, SystemParam)]
pub(crate) struct QueryEntityClasses<'w, 's>(
    Query<
        'w, 's,
        (Entity, &'static Class)
    >
);

#[cfg(feature = "pseudo_class")]
#[derive(SystemParam)]
pub(crate) struct PseudoClassParam<'w, 's>
{
    pub interaction: Query<
        'w, 's,
        (Entity, &'static Interaction),
        Changed<Interaction>
    >,
    pub _children: Query<
        'w, 's,
        (Entity, &'static Children),
        Changed<Children>
    >,
}

#[derive(Deref, SystemParam)]
pub(crate) struct QueryChildren<'w, 's>(
    Query<
        'w, 's,
        &'static Children,
        With<Node>
    >
);