use super::query;
use crate::prelude::StyleSheetAsset;

use bevy::{
    prelude::{
        error, warn, info, debug, trace,
        Deref, DerefMut,
        Handle, Entity,
    },
    utils::HashMap
};

#[derive(Clone)]
pub(super)struct StyleTreeNode
{
    pub sheet_handle: Handle<StyleSheetAsset>,
    pub parent: Option<Handle<StyleSheetAsset>>,
}

#[derive(Default, Deref, DerefMut)]
pub(super) struct StyleTree(
    HashMap<
        Handle<StyleSheetAsset>,
        StyleTreeNode
    >
);

impl StyleTree
{
    fn resolve(
        &self,
        child_node: &Handle<StyleSheetAsset>,
    ) -> Vec<Handle<StyleSheetAsset>> {
        match self.get(child_node)
        {
            Some(style) => {
                let iter = std::iter::once(style.sheet_handle.clone());
                match &style.parent
                {
                    Some(parent) => {
                        self.resolve(parent)
                            .into_iter()
                            .chain(iter)
                            .collect()
                    },
                    None => iter.collect(),
                }
            },
            None => vec![],
        }
    }
}

impl<'me, 'w, 's> StyleTree
{
    fn get_or_find_root(
        &'me mut self,
        entity: Entity,
        query: &'w query::QueryUiNodes<'w, 's>,
    ) -> Option<StyleTreeNode> {
        let (_e, parent, _c, sheet) = match query.get(entity)
        {
            Ok((e, p, c, s)) => (e, p, c, s),
            Err(err) => {
                error!("Query on entity {entity:?} failed, {err}");
                return None;
            },
        };
        
        match (sheet, parent)
        {
            (Some(style), _p) => {
                trace!("Stylesheet found on this node");
                let result = if let Some(node) = self.get(style.handle())
                {
                    trace!("Found existing node entry in tree, returning early");
                    node
                }
                else
                {
                    trace!("Node entry does not exist in tree, creating entry");
                    let parent = match parent {
                        Some(p) => self.get_or_find_root(p.get(), query),
                        None => {
                            debug!("No parent node found, terminating search");
                            None
                        },
                    }.map(|p| p.sheet_handle);
    
                    self.insert_unique_unchecked(
                        style.handle().clone(),
                        StyleTreeNode {
                            sheet_handle: style.handle().clone(),
                            parent
                        }
                    ).1
                };
                Some(result.clone())
            },
            (None, Some(parent)) => self.get_or_find_root(parent.get(), query),
            (None, None) => {
                info!("No parent node provided to find styles");
                None
            }
        }
    }

    pub fn get_styles(
        &'me mut self,
        entity: Entity,
        query: &'w query::QueryUiNodes<'w, 's>,
    ) -> Vec<Handle<StyleSheetAsset>> {
        let root_node = self.get_or_find_root(entity, query);
        match root_node
        {
            Some(node) => {
                self.resolve(&node.sheet_handle)
            },
            None => {
                warn!("No root style-node found");
                vec![]
            }
        }
    }
}
