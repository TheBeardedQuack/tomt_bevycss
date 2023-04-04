use super::{
    Property,
    CacheState,
    CachedProperties,
};
use crate::{
    selector::Selector,
    stylesheet::StyleSheetAsset,
};
use bevy::{
    log::error,
    prelude::{Deref, DerefMut},
    utils::HashMap,
};


/// Internal property cache map. Used by [`Property::apply_system`] to keep track of which properties was already parsed.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct PropertyMeta<T: Property>(HashMap<u64, CachedProperties<T::Cache>>);

impl<T: Property> PropertyMeta<T> {
    /// Gets a cached property value or try to parse.
    ///
    /// If there are some error while parsing, a [`CacheState::Error`] is stored to avoid trying to parse again on next try.
    pub(super) fn get_or_parse(
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
