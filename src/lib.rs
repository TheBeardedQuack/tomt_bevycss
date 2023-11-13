#![doc = include_str!("../README.md")]

mod component;
pub mod error;
mod parser;
pub mod plugins;
pub mod property;
mod selector;
mod stylesheet;
pub mod system;

use crate::{
    property::Property,
    system::{
        sets::BevyCssSet,
        ComponentFilterRegistry,
    },
};

use bevy::{
    ecs::system::SystemState,
    prelude::*,
};

/// use `tomt_bevycss::prelude::*;` to import common components, and plugins and utility functions.
pub mod prelude {
    pub use super::{
        component::{Class, StyleSheet},
        error::BevyCssError,
        plugins::BevyCssPlugin,
        property::{Property, PropertyValues},
        stylesheet::StyleSheetAsset,
        RegisterComponentSelector,
        RegisterProperty,
    };
}

/// Utility trait which adds the [`register_component_selector`](RegisterComponentSelector::register_component_selector)
/// function on [`App`](bevy::prelude::App) to add a new component selector.
///
/// You can register any component you want and name it as you like.
/// It's advised to use `lower-case` and `kebab-case` to match CSS coding style.
///
/// # Examples
///
/// ```rust
/// # use bevy::prelude::*;
/// # use tomt_bevycss::prelude::*;
/// #
/// # fn some_main() {
/// #    let mut app = App::new();
/// #    app.add_plugins(DefaultPlugins).add_plugins(BevyCssPlugin::default());
/// #
///      #[derive(Component)]
///      struct MyFancyComponentSelector;
///
///      app.register_component_selector::<MyFancyComponentSelector>("fancy-pants");
///      // You may use it as selector now, like
///      // fancy-pants {
///      //      background-color: pink;
///      // }
/// # }
/// ```

pub trait RegisterComponentSelector
{
    fn register_component_selector<T>(
        &mut self,
        name: &'static str
    ) -> &mut Self
    where
        T: Component;
}

impl RegisterComponentSelector
for bevy::prelude::App
{
    fn register_component_selector<T>(
        &mut self,
        name: &'static str
    ) -> &mut Self
    where
        T: Component,
    {
        let system_state = SystemState::<Query<Entity, With<T>>>::new(&mut self.world);
        let boxed_state = Box::new(system_state);

        self.world
            .get_resource_or_insert_with::<ComponentFilterRegistry>(|| {
                ComponentFilterRegistry(Default::default())
            })
            .0
            .insert(name, boxed_state);

        self
    }
}

/// Utility trait which adds the [`register_property`](RegisterProperty::register_property) function
/// on [`App`](bevy::prelude::App) to add a [`Property`] parser.
///
/// You need to register only custom properties which implements [`Property`] trait.
pub trait RegisterProperty
{
    fn register_property<T>(
        &mut self
    ) -> &mut Self
    where
        T: Property + 'static;
}

impl RegisterProperty
for bevy::prelude::App
{
    fn register_property<T>(
        &mut self
    ) -> &mut Self
    where
        T: Property + 'static,
    {
        self.add_systems(Update, T::apply_system.in_set(BevyCssSet::Apply))
    }
}
