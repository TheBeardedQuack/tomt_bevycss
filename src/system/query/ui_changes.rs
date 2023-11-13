use crate::prelude::StyleSheet;

use bevy::prelude::{
    Added,
    Changed,
    Entity,
    Node,
    Or,
    Query,
    With,
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

    pub type ReadOnlyWorldQuery = (Or<(Added<StyleSheet>, Changed<StyleSheet>)>, With<Node>);
}

#[cfg(feature = "monitor_changes")]
mod monitor_changes
{
    pub use pseudo_class::ReadOnlyWorldQuery;

    use super::*;
    use crate::prelude::Class;
    use bevy::prelude::{
        Children,
        Parent,
    };

    #[cfg(not(feature = "pseudo_class"))]
    mod pseudo_class
    {
        use super::*;

        pub type ReadOnlyWorldQuery = (
            Or<(
                Added<StyleSheet>,  Changed<StyleSheet>,
                Added<Parent>,      Changed<Parent>,
                Added<Children>,    Changed<Children>,
                Added<Class>,       Changed<Class>,
            )>,
            With<Node>
        );
    }

    #[cfg(feature = "pseudo_class")]
    mod pseudo_class
    {
        use super::*;
        use bevy::prelude::Interaction;

        pub type ReadOnlyWorldQuery = (
            Or<(
                Added<StyleSheet>,  Changed<StyleSheet>,
                Added<Parent>,      Changed<Parent>,
                Added<Children>,    Changed<Children>,
                Added<Class>,       Changed<Class>,
                Added<Interaction>, Changed<Interaction>,
            )>,
            With<Node>
        );
    }
}
