use super::query;
use crate::prelude::StyleSheetAsset;
use bevy::log::*;
use bevy::{
    prelude::{Deref, DerefMut, Entity, Handle},
    utils::HashMap,
};

#[derive(Clone)]
pub(super) struct StyleTreeNode {
    pub entity: Entity,
    pub sheet_handle: Handle<StyleSheetAsset>,
    pub parent: Option<Handle<StyleSheetAsset>>,
}

#[derive(Default, Deref, DerefMut)]
pub(super) struct StyleTree(HashMap<Handle<StyleSheetAsset>, StyleTreeNode>);

impl StyleTree {
    fn resolve(
        &self,
        child_node: &Handle<StyleSheetAsset>,
    ) -> Vec<(Entity, Handle<StyleSheetAsset>)> {
        match self.get(child_node) {
            Some(style) => {
                let iter = std::iter::once((style.entity, style.sheet_handle.clone()));
                match &style.parent {
                    Some(parent) => self.resolve(parent).into_iter().chain(iter).collect(),
                    None => iter.collect(),
                }
            }
            None => vec![],
        }
    }
}

impl<'me, 'w, 's> StyleTree {
    fn get_or_find_root(
        &'me mut self,
        entity: Entity,
        query: &'w query::QueryUiNodes<'w, 's>,
    ) -> Option<StyleTreeNode> {
        let entity_idx = entity.index();

        let (entity, parent, _c, sheet) = match query.get(entity) {
            Ok((e, p, c, s)) => (e, p, c, s),
            Err(err) => {
                error!("Query on entity {entity_idx} failed, {err}");
                return None;
            }
        };

        match (sheet, parent) {
            (Some(style), _p) => {
                trace!("Stylesheet found on entity {entity_idx}");
                let result = if let Some(node) = self.get(style.handle()) {
                    trace!("Entity {entity_idx} is already in the tree, returning early");
                    node
                } else {
                    trace!("Creating entry in tree for entity {entity_idx}");
                    let parent = match parent {
                        Some(p) => self.get_or_find_root(p.get(), query),
                        None => {
                            debug!("Entity {entity_idx} has no parent UI node, terminating search");
                            None
                        }
                    }
                    .map(|p| p.sheet_handle);

                    self.insert_unique_unchecked(
                        style.handle().clone(),
                        StyleTreeNode {
                            entity,
                            sheet_handle: style.handle().clone(),
                            parent,
                        },
                    )
                    .1
                };
                Some(result.clone())
            }
            (None, Some(parent)) => self.get_or_find_root(parent.get(), query),
            (None, None) => {
                debug!("Entity {entity_idx} has no UI parent, or attached stylesheet");
                None
            }
        }
    }

    pub fn get_style_roots_for(
        &'me mut self,
        entity: Entity,
        query: &'w query::QueryUiNodes<'w, 's>,
    ) -> Vec<(Entity, Handle<StyleSheetAsset>)> {
        let root_node = self.get_or_find_root(entity, query);
        match root_node {
            Some(node) => self.resolve(&node.sheet_handle),
            None => vec![],
        }
    }
}
