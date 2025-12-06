use bevy::utils::AHasher;
use cssparser::CowRcStr;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    sync::Mutex,
};

use crate::DynArray;

static RULE_COUNTER: Mutex<usize> = Mutex::new(0);

/// Represents a selector element on a style sheet rule.
/// A single selector can have multiple elements, for instance a selector of `button.enabled`
/// Would generated two elements, one for `button` and another for `.enabled`.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
pub enum SelectorElement
{
    /// A name selector element, like `#score_window`. On CSS used on web, this is as known as id.
    Name(String),

    /// A component selector element, like `window` or `button`
    Component(String),

    /// A class name component selector element, `.border`
    Class(String),

    #[cfg(feature = "pseudo_class")]
    /// A class name component selector element, like `:hover` or `:first-child` or `:empty`
    PseudoClass(String),

    #[cfg(feature = "pseudo_prop")]
    /// A class name component selector element, like `::first-line` or `::first-letter` or `::marker`
    PseudoProp(String),

    /// Indicates a parent-child relation between previous elements and next elements, like `window .border`
    Child,
}

/// A selector parsed from a `css` rule. Each selector has a internal hash used to differentiate between many rules in the same sheet.
#[derive(Clone, Debug, Default)]
pub struct Selector
{
    hash: u64,
    elements: DynArray<SelectorElement>,
    /// Rule loading order from parser
    load_order: usize,
}

impl Selector
{
    /// Creates a new selector for the given elements.
    pub fn new(
        elements: DynArray<SelectorElement>
    ) -> Self {
        let hasher = AHasher::default();

        let hasher = elements.iter().fold(hasher, |mut hasher, el|
        {
            el.hash(&mut hasher);
            hasher
        });

        let hash = hasher.finish();
        Self{
            elements,
            hash,
            load_order: RULE_COUNTER
                .lock()
                .map(|mut lock|
                {
                    *lock += 1;
                    *lock
                })
                .unwrap_or_default(),
        }
    }

    /// Builds a selector tree for this selector.
    /// Each node in the tree is composed of many elements, also each node is parent of the next one.
    pub fn get_parent_tree(
        &self
    ) -> DynArray<DynArray<&SelectorElement>> {
        let mut tree = DynArray::new();
        let mut current_level = DynArray::new();

        for element in &self.elements
        {
            match element
            {
                SelectorElement::Child => {
                    tree.push(current_level);
                    current_level = DynArray::new();
                }
                _ => current_level.push(element),
            }
        }
        tree.push(current_level);

        tree
    }
}

impl std::fmt::Display
for Selector
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        let mut buffer = String::new();

        for element in &self.elements
        {
            match element
            {
                SelectorElement::Name(n) => {
                    buffer.push('#');
                    buffer.push_str(n);
                }

                SelectorElement::Component(c) => {
                    buffer.push_str(c);
                }

                SelectorElement::Class(c) => {
                    buffer.push('.');
                    buffer.push_str(c);
                }

                #[cfg(feature = "pseudo_class")]
                SelectorElement::PseudoClass(c) => {
                    buffer.push(':');
                    buffer.push_str(c);
                }

                #[cfg(feature = "pseudo_prop")]
                SelectorElement::PseudoProp(p) => {
                    buffer.push_str("::");
                    buffer.push_str(p);
                }

                SelectorElement::Child => {
                    buffer.push(' ');
                }
            }
        }

        write!(formatter, "{}", buffer)
    }
}

impl PartialEq
for Selector
{
    fn eq(
        &self,
        other: &Self
    ) -> bool {
        self.hash == other.hash
    }
}

impl Eq
for Selector
{
    // nothing to do
}

impl PartialOrd
for Selector
{
    fn partial_cmp(
        &self,
        other: &Self
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord
for Selector
{
    fn cmp(
        &self,
        other: &Self
    ) -> std::cmp::Ordering {
        match self.elements.len().cmp(&other.elements.len())
        {
            Ordering::Equal => self.load_order.cmp(&other.load_order),
            not_eq => not_eq,
        }
    }
}

impl Hash
for Selector
{
    fn hash<H: Hasher>(
        &self,
        state: &mut H
    ) {
        self.hash.hash(state);
    }
}

impl<'i, IterT: IntoIterator<Item = CowRcStr<'i>>> From<IterT>
for Selector
{
    fn from(
        input: IterT
    ) -> Self {
        let mut elements = DynArray::new();
        let mut next_is_class = false;

        for value in input.into_iter()
            .filter(|v| !v.is_empty())
        {
            if value.as_ref() == "."
            {
                next_is_class = true;
                continue;
            }

            if let Some(value) = value.strip_prefix('#') {
                elements.push(SelectorElement::Name(value.to_string()));
            }
            else if next_is_class {
                elements.push(SelectorElement::Class(value.to_string()))
            }
            else {
                elements.push(SelectorElement::Component(value.to_string()))
            }

            next_is_class = false;
        }

        Self::new(elements)
    }
}
