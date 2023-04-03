use crate::prelude::{
    Class, PseudoClass,
    StyleSheet, StyleSheetAsset,
};
use bevy::{
    prelude::*,
    ecs::system::SystemParam,
};

#[derive(SystemParam)]
pub(crate) struct CssQueryParam<'w, 's> {
    pub assets: Res<'w, Assets<StyleSheetAsset>>,
    pub nodes: Query<
        'w, 's,
        (Entity, Option<&'static Children>, &'static StyleSheet),   // Select
        Changed<StyleSheet>,                                        // Filter
    >,
    pub names: Query<
        'w, 's,
        (Entity, &'static Name)             // Select
    >,
    pub classes: Query<
        'w, 's,
        (Entity, &'static Class)            // Select
    >,
    pub pseudo_classes: Query<
        'w, 's,
        (Entity, &'static PseudoClass)      // Select
    >,
    pub children: Query<
        'w, 's,
        &'static Children,      // Select
        With<Node>              // Filter
    >,
}
