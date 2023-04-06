use crate::prelude::{
    Class,
    StyleSheet,
    StyleSheetAsset,
};

#[cfg(feature = "pseudo_prop")]
use crate::prelude::PseudoProp;

use bevy::{
    prelude::*,
    ecs::system::SystemParam,
};

#[derive(SystemParam)]
pub(crate) struct CssQueryParam<'w, 's>
{
    pub assets: StyleSheetResource<'w>,
    pub nodes: Query<
        'w, 's,
        (Entity, Option<&'static Children>, &'static StyleSheet),
    >,
    pub node_changes: Query<
        'w, 's,
        (Entity, Option<&'static Children>),
        Or<(Changed<StyleSheet>, Changed<Children>, Changed<Interaction>)>
    >,

    pub names: QueryEntityNames<'w, 's>,
    pub classes: QueryEntityClasses<'w, 's>,
    pub children: QueryChildren<'w, 's>,
    
    #[cfg(feature = "pseudo_class")]
    pub pseudo_classes: PseudoClassParam<'w, 's>,
}

#[derive(Deref, SystemParam)]
pub(crate) struct StyleSheetResource<'w>(
    Res<'w, Assets<StyleSheetAsset>>
);

#[derive(Deref, SystemParam,)]
pub(crate) struct QueryEntityNames<'w, 's>(
    Query<
        'w, 's,
        (Entity, &'static Name)
    >
);

#[derive(Deref, SystemParam,)]
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