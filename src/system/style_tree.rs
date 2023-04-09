use super::query::QueryUiChanges;
use crate::prelude::{StyleSheet, StyleSheetAsset};

use bevy::{
    prelude::{
        warn, info,
        Deref, DerefMut,
        Handle, Parent,
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
        parent: Option<&'w Parent>,
        sheet: Option<&'w StyleSheet>,
        query: &'w QueryUiChanges<'w, 's>
    ) -> Option<StyleTreeNode> {
        if let Some(style) = sheet
        {
            if let Some(node) = self.get(style.handle())
            {
                Some(node.clone())
            }
            else
            {
                let parent = match parent {
                    Some(p) => match query.get(p.get())
                    {
                        Ok((_, p, _, s)) => self.get_or_find_root(p, s, query),
                        Err(_) => {
                            info!("No parent node found, terminating search");
                            None
                        },
                    },
                    None => None,
                }.map(|p| p.sheet_handle);

                Some(self.insert_unique_unchecked(
                    style.handle().clone(),
                    StyleTreeNode {
                        sheet_handle: style.handle().clone(),
                        parent
                    }
                ).1.clone())
            }
        }
        else
        {
            match parent {
                Some(p) => match query.get(p.get()) {
                    Ok((_, p, _, s)) => self.get_or_find_root(p, s, query),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }

    pub fn get_styles(
        &'me mut self,
        parent: Option<&'w Parent>,
        sheet: Option<&'w StyleSheet>,
        query: &'w QueryUiChanges<'w, 's>
    ) -> Vec<Handle<StyleSheetAsset>> {
        let root_node = self.get_or_find_root(parent, sheet, query);
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
