mod cache_state;
pub use cache_state::*;

mod cached_properties;
pub use cached_properties::*;

mod colors;

mod property_meta;
pub use property_meta::*;

mod property_token;
pub use property_token::*;

mod property_values;
pub use property_values::*;

mod selected_entities;
pub use selected_entities::*;

mod stylesheet_state;
pub use stylesheet_state::*;

pub(crate) mod impls;
/// Impls for `bevy_text` [`Text`] component
pub(crate) mod text;


use crate::prelude::{
    BevyCssError,
    StyleSheetAsset,
};

use bevy::{
    ecs::query::{
        QueryItem,
        ReadOnlyWorldQuery,
        WorldQuery,
    },
    prelude::{
        Assets, AssetServer,
        Commands,
        Local,
        Query,
        Res,
    },
};
use std::any::Any;

/// Determines how a property should interact and modify the [ecs world](`bevy::prelude::World`).
///
/// Each implementation of this trait should be registered with [`RegisterProperty`](crate::RegisterProperty) trait, where
/// will be converted into a `system` and run whenever a matched, specified by [`name()`](`Property::name()`) property is found.
///
/// These are the associated types that must by specified by implementors:
/// - [`Cache`](Property::Cache) is a cached value to be applied by this trait.
/// On the first time the `system` runs it'll call [`parse`](`Property::parse`) and cache the value.
/// Subsequential runs will only fetch the cached value.
/// - [`Components`](Property::Components) is which components will be send to [`apply`](`Property::apply`) function whenever a
/// valid cache exists and a matching property was found on any sheet rule. Check [`WorldQuery`] for more.
/// - [`Filters`](Property::Filters) is used to filter which entities will be applied the property modification.
/// Entities are first filtered by [`selectors`](`crate::selector::Selector`), but it can be useful to also ensure some behavior for safety reasons,
/// like only inserting [`TextAlignment`](bevy::prelude::TextAlignment) if the entity also has a [`Text`](bevy::prelude::Text) component.
///  Check [`WorldQuery`] for more.
///
/// These are tree functions required to be implemented:
/// - [`name`](Property::name) indicates which property name should matched for.
/// - [`parse`](Property::parse) parses the [`PropertyValues`] into the [`Cache`](Property::Cache) value to be reused across multiple entities.
/// - [`apply`](Property::apply) applies on the given [`Components`](Property::Components) the [`Cache`](Property::Cache) value.
/// Additionally, an [`AssetServer`] and [`Commands`] parameters are provided for more complex use cases.
///
/// Also, there one function which have default implementations:
/// - [`apply_system`](Property::apply_system) is a [`system`](https://docs.rs/bevy_ecs/0.8.1/bevy_ecs/system/index.html) which interacts with
/// [ecs world](`bevy::prelude::World`) and call the [`apply`](Property::apply) function on every matched entity.
pub trait Property:
    Default + Sized + Send + Sync + 'static
{
    /// The cached value type to be applied by property.
    type Cache: Default + Any + Send + Sync;
    /// Which components should be queried when applying the modification. Check [`WorldQuery`] for more.
    type Components: WorldQuery;
    /// Filters conditions to be applied when querying entities by this property. Check [`ReadOnlyWorldQuery`] for more.
    type Filters: ReadOnlyWorldQuery;

    /// Indicates which property name should matched for. Must match the same property name as on `css` file.
    ///
    /// For compliance, use always `lower-case` and `kebab-case` names.
    fn name(
        // no args
    ) -> &'static str;

    /// Parses the [`PropertyValues`] into the [`Cache`](Property::Cache) value to be reused across multiple entities.
    ///
    /// This function is called only once, on the first time a matching property is found while applying style rule.
    /// If an error is returned, it is also cached so no more attempt are made.
    fn parse(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError>;

    /// Applies on the given [`Components`](Property::Components) the [`Cache`](Property::Cache) value.
    /// Additionally, an [`AssetServer`] and [`Commands`] parameters are provided for more complex use cases.
    ///
    /// If mutability is desired while applying the changes, declare [`Components`](Property::Components) as mutable.
    fn apply(
        cache: &Self::Cache,
        components: QueryItem<Self::Components>,
        asset_server: &AssetServer,
        commands: &mut Commands,
    );

    /// The [`system`](https://docs.rs/bevy_ecs/0.8.1/bevy_ecs/system/index.html) which interacts with
    /// [ecs world](`bevy::prelude::World`) and call [`apply`](Property::apply) function on every matched entity.
    ///
    /// The default implementation will cover most use cases, by just implementing [`apply`](Property::apply)
    fn apply_system(
        mut local: Local<PropertyMeta<Self>>,
        assets: Res<Assets<StyleSheetAsset>>,
        apply_sheets: Res<StyleSheetState>,
        mut q_nodes: Query<Self::Components, Self::Filters>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        for (entity, style) in apply_sheets.iter()
        {
            let source = match style.get_key_value(Self::name())
            {
                Some((_prop, source)) => source,
                None => continue,
            };

            let rules = match assets.get(&source.styleheet)
            {
                Some(asset) => asset,
                None => continue,
            };

            let cached_value = match local.get_or_parse(rules, &source.selector)
            {
                CacheState::Ok(cached) => cached,
                _other => continue,
            };

            let components = match q_nodes.get_mut(*entity)
            {
                Ok(cmp) => cmp,
                Err(_) => continue,
            };

            Self::apply(cached_value, components, &asset_server, &mut commands);
        }
    }
}
