use bevy::utils::AHasher;
use cssparser::CowRcStr;
use smallvec::{smallvec, SmallVec};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{Add, AddAssign},
    sync::Mutex,
};

static RULE_COUNTER: Mutex<usize> = Mutex::new(0);

#[derive(Clone, Copy, Debug, Default)]
#[derive(PartialEq, Eq, Hash)]
pub struct RuleWeight
{
    pub ids: u32,
    pub classes: u32,
    pub types: u32,
}

impl RuleWeight
{
    pub const ID: Self = Self { ids: 1, classes: 0, types: 0 };
    pub const CLASS: Self = Self { ids: 0, classes: 1, types: 0 };
    pub const TYPE: Self = Self { ids: 0, classes: 0, types: 1 };
}

impl Add
for RuleWeight
{
    type Output = Self;

    fn add(
        self,
        rhs: Self
    ) -> Self::Output {
        Self {
            ids: self.ids + rhs.ids,
            classes: self.classes + rhs.classes,
            types: self.types + rhs.types,
        }
    }
}

impl AddAssign
for RuleWeight
{
    fn add_assign(
        &mut self,
        rhs: Self
    ) {
        self.ids += rhs.ids;
        self.classes += rhs.classes;
        self.types += rhs.types;
    }
}

impl Sum
for RuleWeight
{
    fn sum<I: Iterator<Item = Self>>(
        iter: I
    ) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

impl Ord
for RuleWeight
{
    fn cmp(
        &self,
        other: &Self
    ) -> Ordering {
        self.ids.cmp(&other.ids)
            .then_with(|| self.classes.cmp(&other.classes))
            .then_with(|| self.types.cmp(&other.types))
    }
}

impl PartialOrd
for RuleWeight
{
    fn partial_cmp(
        &self,
        other: &Self
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

impl SelectorElement
{
    pub fn rule_weight(
        &self
    ) -> RuleWeight {
        match self
        {
            // Named elements (1-0-0)
            SelectorElement::Name(_) => RuleWeight::ID,

            // Class elements (0-1-0)
            SelectorElement::Class(_) => RuleWeight::CLASS,
            #[cfg(feature = "pseudo_class")]
            SelectorElement::PseudoClass(_) => RuleWeight::CLASS,

            // Type elements (0-0-1)
            SelectorElement::Component(_) => RuleWeight::TYPE,
            #[cfg(feature = "pseudo_prop")]
            SelectorElement::PseudoProp(_) => RuleWeight::TYPE,

            // Child elements (zero-weight)
            SelectorElement::Child => RuleWeight::default(),
        }
    }
}

/// A selector parsed from a `css` rule. Each selector has a internal hash used to differentiate between many rules in the same sheet.
#[derive(Clone, Debug, Default)]
pub struct Selector
{
    hash: u64,
    elements: SmallVec<[SelectorElement; 8]>,
    /// Rule loading order from parser
    load_order: usize,
}

impl Selector
{
    /// Creates a new selector for the given elements.
    pub fn new(
        elements: SmallVec<[SelectorElement; 8]>
    ) -> Self {
        let hasher = AHasher::default();

        let hasher = elements.iter()
            .fold(hasher, |mut hasher, el|
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
    ) -> SmallVec<[SmallVec<[&SelectorElement; 8]>; 8]> {
        let mut tree = SmallVec::new();
        let mut current_level = SmallVec::new();

        for element in &self.elements
        {
            match element
            {
                SelectorElement::Child => {
                    tree.push(current_level);
                    current_level = SmallVec::new();
                }
                _ => current_level.push(element),
            }
        }
        tree.push(current_level);

        tree
    }

    pub fn rule_weight(
        &self
    ) -> RuleWeight {
        self.elements.iter()
            .map(|rule| rule.rule_weight())
            .sum()
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
        match self.rule_weight().cmp(&other.rule_weight())
        {
            Ordering::Equal => self.load_order.cmp(&other.load_order),
            not_eq => not_eq
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

impl<'i> From<Vec<CowRcStr<'i>>>
for Selector
{
    fn from(
        input: Vec<CowRcStr<'i>>
    ) -> Self {
        let mut elements = smallvec![];
        let mut next_is_class = false;

        for value in input.into_iter()
            .filter(|v| !v.is_empty())
        {
            if value.as_ref() == "."
            {
                next_is_class = true;
                continue;
            }

            if let Some(value) = value.strip_prefix('#')
            {
                elements.push(SelectorElement::Name(value.to_string()));
            }
            else if next_is_class
            {
                elements.push(SelectorElement::Class(value.to_string()))
            }
            else
            {
                elements.push(SelectorElement::Component(value.to_string()))
            }

            next_is_class = false;
        }

        Self::new(elements)
    }
}
