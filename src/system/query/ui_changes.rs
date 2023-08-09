use crate::prelude::StyleSheet;

use bevy::prelude::{
    Changed,
    Entity,
    Query,
};

pub type QueryUiChanges<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = Entity;
pub use monitor_changes::ReadOnlyWorldQuery;

#[cfg(not(feature = "monitor_changes"))]
mod monitor_changes
{
    use super::*;

    pub type ReadOnlyWorldQuery = Changed<StyleSheet>;
}

#[cfg(feature = "monitor_changes")]
mod monitor_changes
{
    pub use pseudo_class::ReadOnlyWorldQuery;

    use super::*;
    use crate::prelude::Class;
    use bevy::prelude::{
        Children,
        Or,
    };

    #[cfg(not(feature = "pseudo_class"))]
    mod pseudo_class
    {
        use super::*;

        pub type ReadOnlyWorldQuery = Or<(
            Changed<StyleSheet>,
            Changed<Children>,
            Changed<Class>,
        )>;
    }

    #[cfg(feature = "pseudo_class")]
    mod pseudo_class
    {
        use super::*;
        use bevy::prelude::Interaction;

        pub type ReadOnlyWorldQuery = Or<(
            Changed<StyleSheet>,
            Changed<Children>,
            Changed<Class>,
            Changed<Interaction>,
        )>;
    }
}
