use crate::prelude::{
    Class,
    StyleSheet,
    StyleSheetAsset,
};

#[cfg(feature = "pseudo_class")]
use crate::prelude::PseudoClass;

#[cfg(feature = "pseudo_prop")]
use crate::prelude::PseudoProp;

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
    #[cfg(feature = "pseudo_class")]
    pub pseudo_classes: Query<
        'w, 's,
        (Entity, &'static PseudoClass)      // Select
    >,
    #[cfg(feature = "pseudo_prop")]
    pub pseudo_props: Query<
        'w, 's,
        (Entity, &'static PseudoProp)
    >,
    pub children: Query<
        'w, 's,
        &'static Children,      // Select
        With<Node>              // Filter
    >,
}
