use super::StyleRule;
use crate::{
    parser::{
        StyleSheetParser,
        StyleSheetType
    },
    property::PropertyValues,
    selector::Selector,
};

use std::hash::{Hash, Hasher};
use smallvec::SmallVec;
use bevy::{
    reflect::{
        TypeUuid,
        TypePath,
    },
    log::trace,
    utils::AHasher,
};

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "14b98dd6-5425-4692-a561-5e6ae9180554"]
/// A cascading style sheet (`css`) asset file.
///
/// _Note_: This asset only store intermediate data, like rules and properties.
/// The parsing to final ECS component values is done by a internal `exclusive_system` and is
/// cached on [`Local`](bevy::prelude::Local) resources, which isn't possible to get outside the system.
pub struct StyleSheetAsset {
    path: String,
    hash: u64,
    rules: SmallVec<[StyleRule; 8]>,
}

impl StyleSheetAsset
{
    #[inline]
    fn parse_as_sass(
        path: &str
    ) -> bool {
        #[cfg(feature = "sass")]
        { path.ends_with(".scss") }
        
        #[cfg(not(feature = "sass"))]
        { false }
    }

    /// Parses a string with a valid CSS into a list of [`crate::stylesheet::StyleRule`]s.
    ///
    /// This used by internal asset loader to keep track of where each asset came from.
    /// If you are creating this struct by hand, you can safely supply an  empty string as path.
    pub fn parse(
        path: &str,
        content: &str
    ) -> Self {
        trace!("StyleSheetAsset::parse");

        let mut hasher = AHasher::default();
        content.hash(&mut hasher);
        let hash = hasher.finish();

        let mut parse_mode = StyleSheetType::default();
        if Self::parse_as_sass(path)
        {
            parse_mode = StyleSheetType::Sass;
        }

        Self {
            path: path.to_string(),
            hash,
            rules: StyleSheetParser::parse(content, parse_mode)
        }
    }

    /// Returns the list of properties defined by the given [`crate::selector::Selector`].
    pub fn get_property_names(
        &self,
        selector: &Selector
    ) -> Option<Vec<String>> {
        self.rules.iter()
            .find(|&rule| rule.selector == *selector)
            .map(|rule|
                rule.properties.iter()
                    .map(|(prop, _val)| prop.clone())
                    .collect::<Vec<String>>()
            )
    }

    /// Returns the [`PropertyValues`] on the given [`crate::selector::Selector`] with the given name.
    pub fn get_property_value(
        &self,
        selector: &Selector,
        name: &str
    ) -> Option<&PropertyValues> {
        self.rules
            .iter()
            .find(|&rule| &rule.selector == selector)
            .and_then(|rule| rule.properties.get(name))
    }

    /// Iterates over all existing rules
    pub fn iter(
        &self
    ) -> impl Iterator<Item = &StyleRule> {
        self.rules.iter()
    }

    /// Internal hash computed from content and used for equality and ordering comparison
    pub fn hash(
        &self
    ) -> u64 {
        self.hash
    }

    /// Asset path, for debug reasons only
    pub fn path(
        &self
    ) -> &str {
        &self.path
    }
}
