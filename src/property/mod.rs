mod colors;
pub(crate) mod impls;

mod property_token;
pub use property_token::*;

mod property_values;
pub use property_values::*;

use crate::{
    prelude::{
        BevyCssError,
        StyleSheetAsset,
    },
    selector::Selector,
};

use std::any::Any;

use smallvec::SmallVec;

use bevy::{
    ecs::query::{QueryItem, ReadOnlyWorldQuery, WorldQuery},
    prelude::{
        error, trace, AssetServer, Assets, Commands, Deref, DerefMut, Entity, Handle, Local,
        Query, Res, Resource,
    },
    utils::HashMap,
};

/// Internal cache state. Used by [`CachedProperties`] to avoid parsing properties of the same rule on same sheet.
#[derive(Default, Debug, Clone)]
pub enum CacheState<T> {
    /// No parse was performed yet
    #[default]
    None,
    /// Parse was performed and yielded a valid value.
    Ok(T),
    /// Parse was performed but returned an error.
    Error,
}

/// Internal cache map. Used by [`PropertyMeta`] to keep track of which properties was already parsed.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct CachedProperties<T>(HashMap<Selector, CacheState<T>>);

/// Internal property cache map. Used by [`Property::apply_system`] to keep track of which properties was already parsed.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct PropertyMeta<T: Property>(HashMap<u64, CachedProperties<T::Cache>>);

impl<T: Property> PropertyMeta<T> {
    /// Gets a cached property value or try to parse.
    ///
    /// If there are some error while parsing, a [`CacheState::Error`] is stored to avoid trying to parse again on next try.
    fn get_or_parse(
        &mut self,
        rules: &StyleSheetAsset,
        selector: &Selector,
    ) -> &CacheState<T::Cache> {
        let cached_properties = self.entry(rules.hash()).or_default();

        // Avoid using HashMap::entry since it requires ownership of key
        if cached_properties.contains_key(selector) {
            cached_properties.get(selector).unwrap()
        } else {
            let new_cache = rules
                .get_properties(selector, T::name())
                .map(|values| match T::parse(values) {
                    Ok(cache) => CacheState::Ok(cache),
                    Err(err) => {
                        error!("Failed to parse property {}. Error: {}", T::name(), err);
                        // TODO: Clear cache state when the asset is reloaded, since values may be changed.
                        CacheState::Error
                    }
                })
                .unwrap_or(CacheState::None);

            cached_properties.insert(selector.clone(), new_cache);
            cached_properties.get(selector).unwrap()
        }
    }
}

/// Maps which entities was selected by a [`Selector`]
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct SelectedEntities(HashMap<Selector, SmallVec<[Entity; 8]>>);

/// Maps sheets for each [`StyleSheetAsset`].
#[derive(Debug, Clone, Default, Deref, DerefMut, Resource)]
pub struct StyleSheetState(HashMap<Handle<StyleSheetAsset>, SelectedEntities>);

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
/// Entities are first filtered by [`selectors`](`Selector`), but it can be useful to also ensure some behavior for safety reasons,
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
pub trait Property: Default + Sized + Send + Sync + 'static {
    /// The cached value type to be applied by property.
    type Cache: Default + Any + Send + Sync;
    /// Which components should be queried when applying the modification. Check [`WorldQuery`] for more.
    type Components: WorldQuery;
    /// Filters conditions to be applied when querying entities by this property. Check [`ReadOnlyWorldQuery`] for more.
    type Filters: ReadOnlyWorldQuery;

    /// Indicates which property name should matched for. Must match the same property name as on `css` file.
    ///
    /// For compliance, use always `lower-case` and `kebab-case` names.
    fn name() -> &'static str;

    /// Parses the [`PropertyValues`] into the [`Cache`](Property::Cache) value to be reused across multiple entities.
    ///
    /// This function is called only once, on the first time a matching property is found while applying style rule.
    /// If an error is returned, it is also cached so no more attempt are made.
    fn parse(values: &PropertyValues) -> Result<Self::Cache, BevyCssError>;

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
        for (handle, selected) in apply_sheets.iter() {
            if let Some(rules) = assets.get(handle) {
                for (selector, entities) in selected.iter() {
                    if let CacheState::Ok(cached) = local.get_or_parse(rules, selector) {
                        trace!(
                            r#"Applying property "{}" from sheet "{}" ({})"#,
                            Self::name(),
                            rules.path(),
                            selector
                        );
                        for entity in entities {
                            if let Ok(components) = q_nodes.get_mut(*entity) {
                                Self::apply(cached, components, &asset_server, &mut commands);
                            }
                        }
                    }
                }
            }
        }
    }
}
