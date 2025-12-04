mod component_filter;

pub(crate) use component_filter::*;

mod component_filter_registry;
pub(crate) use component_filter_registry::*;

mod css_query_param;
pub(crate) use css_query_param::*;

pub(crate) mod query;

pub mod sets;

mod style_tree;
use style_tree::StyleTree;

use crate::{
    component::{
        MatchSelectorElement,
        StyleSheet,
    },
    property::{StyleSheetState, StyleSheetStateBuilder},
    selector::{Selector, SelectorElement},
    stylesheet::StyleSheetAsset,
};

use bevy::{
    ecs::system::SystemState,
    log::{error, debug, trace},
    prelude::{
        AssetEvent, Assets,
        Children, Component,
        Deref, DerefMut,
        Entity, EventReader,
        Mut,
        Parent,
        Query,
        ResMut, Resource,
        World,
    },
};
use smallvec::{smallvec, SmallVec};

#[derive(Deref, DerefMut, Resource)]
pub(crate) struct PrepareParams(
    SystemState<CssQueryParam<'static, 'static>>
);

impl PrepareParams
{
    pub fn new(
        world: &mut World
    ) -> Self {
        Self(SystemState::new(world))
    }
}

/// Exclusive system which selects all entities and prepare the internal state used by [`Property`](crate::Property) systems.
pub(crate) fn prepare(
    world: &mut World
) {
    world.resource_scope(|world, mut params: Mut<PrepareParams>|
    {
        world.resource_scope(|world, mut registry: Mut<ComponentFilterRegistry>|
        {
            let assets = world.resource::<Assets<StyleSheetAsset>>();
            let css_query = params.get(world);
            let state = prepare_state(world, assets, css_query, &mut registry);

            if !state.is_empty()
            {
                let mut state_res = world
                    .get_resource_mut::<StyleSheetState>()
                    .expect("Should be added by plugin");

                *state_res = state;
            }
        });
    });
}

/// Prepare state to be used by [`Property`](crate::Property) systems
pub(crate) fn prepare_state(
    world: &World,
    assets: &Assets<StyleSheetAsset>,
    params: CssQueryParam,
    registry: &mut ComponentFilterRegistry
) -> StyleSheetState {
    let mut state = StyleSheetStateBuilder::default();
    let mut style_tree: StyleTree = Default::default();

    // Find only changed components
    for updated_entity in &params.ui_changes
    {
        debug!("Update detected for entity {}", updated_entity.index());

        // Find list of stylesheets that apply to this component (and cache in style_tree for next iterations)
        for (root_entity, sheet_handle) in style_tree
            .get_style_roots_for(updated_entity, &params.ui_nodes)
            .iter()
        {
            let style_sheet = match params.assets.get(sheet_handle)
            {
                Some(sheet) => sheet,
                None => {
                    error!("Failed to load stylesheet from handle {sheet_handle:?}");
                    continue;
                }
            };

            debug!("Applying style {}", style_sheet.path());
            for rule in style_sheet.iter()
            {
                let mut entities = select_entities(
                    *root_entity,
                    updated_entity,
                    &rule.selector,
                    world,
                    &params,
                    registry,
                );

                trace!(
                    "Applying rule '{}' on {} entities",
                    rule.selector.to_string(),
                    entities.len()
                );

                let existing_state = state.entry(sheet_handle.clone())
                    .or_default()
                    .entry(rule.selector.clone())
                    .or_default();

                entities = entities.into_iter()
                    .filter(|e| !existing_state.contains(e))
                    .collect();
                existing_state.append(&mut entities);
            }
        }
    }

    if state.len() > 0
    {
        trace!("PreProcess result: {state:?}");
    }
    state.build(assets)
}

fn build_entity_filter(
    root: Entity,
    updated_node: Entity,
    css_query: &CssQueryParam
) -> Option<SmallVec<[Entity; 8]>> {
    css_query.ui_nodes.get(updated_node)
        .map(|(_entity, parent, children, _stylesheet)|
        {
            // Add parents recursively
            parent.map_or_else(SmallVec::default, |parent| 
                    get_parents_recursively(root, parent, &css_query.parent)
                )
                .into_iter()
                // Add the entity that triggered the change
                .chain(std::iter::once(updated_node))
                // Add children recursively
                .chain(children.map_or_else(SmallVec::default, |children|
                    get_children_recursively(children, &css_query.children)
                ))
                .collect()
        })
        .ok()
}

/// Select all entities using the given [`Selector`](crate::selector::Selector).
///
/// If no [`Children`] is supplied, then the selector is applied only on root entity.
fn select_entities(
    root_node: Entity,
    updated_node: Entity,
    selector: &Selector,
    world: &World,
    css_query: &CssQueryParam,
    registry: &mut ComponentFilterRegistry
) -> SmallVec<[Entity; 8]> {
    let mut parent_tree = selector.get_parent_tree();

    if parent_tree.is_empty()
    {
        return SmallVec::new();
    }

    let mut filter = build_entity_filter(root_node, updated_node, css_query);
    loop
    {
        // TODO: Rework this to use a index to avoid recreating parent_tree every time the systems runs.
        // This is has little to no impact on performance, since this system doesn't runs often.
        let node = parent_tree.remove(0);
        let entities = select_entities_node(node, world, css_query, registry, filter.clone());

        if parent_tree.is_empty()
        {
            break entities;
        }
        else
        {
            let children = entities.into_iter()
                .filter_map(|e| css_query.children.get(e).ok())
                .flat_map(|(_e, ch)|
                    get_children_recursively(ch, &css_query.children)
                )
                .collect();

            filter = Some(children);
        }
    }
}

/// Filter entities matching the given selectors.
/// This function is called once per node on tree returned by [`get_parent_tree`](Selector::get_parent_tree)
fn select_entities_node(
    node: SmallVec<[&SelectorElement; 8]>,
    world: &World,
    css_query: &CssQueryParam,
    registry: &mut ComponentFilterRegistry,
    filter: Option<SmallVec<[Entity; 8]>>
) -> SmallVec<[Entity; 8]> {
    let fold_fn = |
        filter: Option<SmallVec<[Entity; 8]>>,
        element: &SelectorElement
    | -> Option<SmallVec<[Entity; 8]>> {
        let result = match element
        {
            SelectorElement::Name(name) => get_entities_with(
                name.as_str(),
                &css_query.names,
                filter
            ),

            SelectorElement::Class(class) => get_entities_with(
                class.as_str(),
                &css_query.classes,
                filter
            ),

            #[cfg(feature = "pseudo_class")]
            SelectorElement::PseudoClass(class) => get_entities_with_pseudo_class(
                class.as_str(),
                &css_query.pseudo_classes,
                filter
            ),

            #[cfg(feature = "pseudo_prop")]
            SelectorElement::PseudoProp(_prop) => todo!(
                "Implement PseudoProperty selection"
            ),

            SelectorElement::Component(component) => get_entities_with_component(
                component.as_str(),
                world,
                registry,
                filter
            ),

            // All child elements are filtered by [`get_parent_tree`](Selector::get_parent_tree)
            SelectorElement::Child => unreachable!(),
        };

        Some(result)
    };

    node.into_iter()
        .fold(filter, fold_fn)
        .unwrap_or_default()
}

#[cfg(feature = "pseudo_class")]
fn get_entities_with_pseudo_class(
    name: &str,
    query: &PseudoClassParam,
    filter: Option<SmallVec<[Entity; 8]>>
) -> SmallVec<[Entity; 8]> {
    use bevy::prelude::Interaction;

    let mut buffer: SmallVec<[Entity; 8]> = Default::default();
    for (entity, action) in query.interaction.iter()
    {
        match (name, *action)
        {
            ("hover", Interaction::Hovered) => trace!("Entity[{entity:?}]:hover"),
            ("click", Interaction::Pressed) => trace!("Entity[{entity:?}]:click"),
            _ => continue,
        };

        match &filter
        {
            Some(f) if !f.contains(&entity) => {
                trace!("Entity {entity:?} discarded by filter");
                continue;
            }
            _ => buffer.push(entity),
        }
    }

    buffer
}

/// Utility function to filter any entities by using a component with implements [`MatchSelectorElement`]
fn get_entities_with<T>(
    name: &str,
    query: &Query<(Entity, &'static T)>,
    filter: Option<SmallVec<[Entity; 8]>>
) -> SmallVec<[Entity; 8]>
where
    T: Component + MatchSelectorElement,
{
    query.iter()
        .filter_map(|(e, rhs)| match rhs.matches(name)
        {
            true => Some(e),
            false => None,
        })
        .filter(|e| match &filter
        {
            Some(filter) => filter.contains(e),
            None => true,
        })
        .collect()
}

/// Filters entities which have the components specified on selector, like "a" or "button".
///
/// The component must be registered on [`ComponentFilterRegistry`]
fn get_entities_with_component(
    name: &str,
    world: &World,
    components: &mut ComponentFilterRegistry,
    filter: Option<SmallVec<[Entity; 8]>>
) -> SmallVec<[Entity; 8]> {
    match components.0.get_mut(name)
    {
        Some(query) => {
            let mut buffer = query.filter(world);
            if let Some(filter) = &filter
            {
                buffer = buffer.into_iter()
                    .filter(|e| filter.contains(e))
                    .collect();
            }

            buffer
        }
        None => {
            error!("Unregistered component selector {}", name);
            SmallVec::new()
        }
    }
}

/// Starting with the provided [Parent], collect all UI parent entities, recurisevely up the entity tree
/// # Arguments
/// `root` - The top-level [Entity] which contains the stylesheet, passed in to provide early stop when root hit
/// `parent` - First [Parent] component to start search with (appears last in returned list)
/// `query_parent` - Bevy [Query] paramter to perform recursive searching
fn get_parents_recursively(
    root: Entity,
    parent: &Parent,
    query_parent: &query::QueryEntityParent
) -> SmallVec<[Entity; 8]> {
    let mut result = match query_parent.get(parent.get())
    {
        Ok((entity, parent)) => match entity == root
        {
            true => smallvec![entity],
            false => get_parents_recursively(root, parent, query_parent),
        },
        Err(_err) => Default::default(),
    };

    result.push(parent.get());
    result
}

/// Starting with the provided [Children] component, collect all UI children entities, recursively down the entity tree
/// # Arguments
/// `children` First [Children] component to start search with (children appear depth first in returned list)
/// `query_children` - Bevy [Query] parameter to perform recursive searching with
fn get_children_recursively(
    children: &Children,
    query_childs: &query::QueryEntityChildren,
) -> SmallVec<[Entity; 8]> {
    children
        .iter()
        .flat_map(|&e|
            std::iter::once(e).chain(
                query_childs.get(e)
                    .map_or(SmallVec::new(), |(_c, gc)|
                        get_children_recursively(gc, query_childs)
                    )
            )
        )
        .collect()
}

/// Auto reapply style sheets when hot reloading is enabled
pub(crate) fn hot_reload_style_sheets(
    mut assets_events: EventReader<AssetEvent<StyleSheetAsset>>,
    mut q_sheets: Query<&mut StyleSheet>,
) {
    for evt in assets_events.read()
    {
        if let AssetEvent::Modified { id } = evt
        {
            q_sheets.iter_mut()
                .filter(|sheet| &sheet.handle().id() == id)
                .for_each(|mut sheet|
                {
                    debug!("Refreshing sheet {:?}", sheet);
                    sheet.refresh();
                });
        }
    }
}

/// Clear temporary state
pub(crate) fn clear_state(
    mut sheet_rule: ResMut<StyleSheetState>
) {
    if sheet_rule.len() > 0
    {
        debug!("Finished applying style sheet.");
        sheet_rule.clear();
    }
}
