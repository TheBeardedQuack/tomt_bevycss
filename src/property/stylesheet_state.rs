use super::SelectedEntities;
use crate::{selector::Selector, stylesheet::StyleSheetAsset};
use bevy::{
    prelude::{Assets, Deref, DerefMut, Entity, Handle, Resource},
    utils::HashMap,
};

#[derive(Debug, Clone)]
pub struct StyleSource {
    pub styleheet: Handle<StyleSheetAsset>,
    pub selector: Selector,
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct ComputedStyle(HashMap<String, StyleSource>);

/// Maps sheets for each [`StyleSheetAsset`].
#[derive(Debug, Clone, Default, Deref, DerefMut, Resource)]
pub struct StyleSheetStateBuilder(HashMap<Handle<StyleSheetAsset>, SelectedEntities>);

#[derive(Debug, Clone, Default, Deref, DerefMut, Resource)]
pub struct StyleSheetState(HashMap<Entity, ComputedStyle>);

impl StyleSheetStateBuilder {
    pub(crate) fn build(&mut self, assets: &Assets<StyleSheetAsset>) -> StyleSheetState {
        let mut result = StyleSheetState::default();

        for (handle, selected) in self.iter() {
            if let Some(sheet) = assets.get(handle) {
                // Invert list of entities for each selector, into a list of selectors for each entity
                let mut inverted: HashMap<Entity, Vec<Selector>> = HashMap::new();
                for (selector, entities) in selected.iter() {
                    for entity in entities.iter() {
                        inverted.entry(*entity).or_default().push(selector.clone());
                    }
                }

                // "Pre-apply" the selectors to get a list of properties without duplicates
                for (entity, mut selectors) in inverted {
                    let style = result.entry(entity).or_default();

                    selectors.sort();
                    for selector in selectors.iter() {
                        for prop in sheet.get_property_names(selector).unwrap_or_default() {
                            style.insert(
                                prop,
                                StyleSource {
                                    styleheet: handle.clone(),
                                    selector: selector.clone(),
                                },
                            );
                        }
                    }
                }
            }
        }

        result
    }
}
