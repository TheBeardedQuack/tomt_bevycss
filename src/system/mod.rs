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

use bevy::{
    ecs::system::SystemState,
    prelude::{
        error, debug, trace,
        Assets, AssetEvent, Children, Component, Deref, DerefMut,
        Entity, EventReader, Interaction, Mut,
        Query, ResMut, Resource, World,
    },
};
use smallvec::SmallVec;

use crate::{
    component::{
        MatchSelectorElement,
        StyleSheet
    },
    property::{
        StyleSheetState,
        StyleSheetStateBuilder,
    },
    selector::{
        Selector,
        SelectorElement
    },
    stylesheet::StyleSheetAsset,
};

#[derive(Deref, DerefMut, Resource)]
pub(crate) struct PrepareParams(SystemState<CssQueryParam<'static, 'static>>);

impl PrepareParams {
    pub fn new(
        world: &mut World
    ) -> Self {
        Self(SystemState::new(world))
    }
}

/// Exclusive system which selects all entities and prepare the internal state used by [`Property`](crate::Property) systems.
pub(crate) fn prepare(
    world: &mut World,
) {
    world.resource_scope(|world, mut params: Mut<PrepareParams>| {
        world.resource_scope(|world, mut registry: Mut<ComponentFilterRegistry>| {
            let assets = world.resource::<Assets<StyleSheetAsset>>();
            let css_query = params.get(world);
            let state = prepare_state(world, assets, css_query, &mut registry);

            if !state.is_empty() {
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
    registry: &mut ComponentFilterRegistry,
) -> StyleSheetState {
    let mut state = StyleSheetStateBuilder::default();
    let mut style_tree: StyleTree = Default::default();

    let mut handled_roots: SmallVec<[Entity; 8]> = Default::default();

    // Find only changed components
    for updated_entity in &params.ui_changes
    {
        // Find list of stylesheets that apply to this component (and cache in style_tree for next iterations)
        for (root_entity, sheet_handle) in style_tree
            .get_style_roots_for(updated_entity, &params.ui_nodes)
            .iter()
        {
            if handled_roots.contains(root_entity) { continue; }

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
                let root_children = params.children.get(*root_entity)
                    .map(|(_e, c)| c)
                    .ok();

                let entities =
                    select_entities(*root_entity, root_children, &rule.selector, world, &params, registry);

                trace!(
                    "Applying rule ({}) on {} entities",
                    rule.selector.to_string(),
                    entities.len()
                );

                state
                    .entry(sheet_handle.clone())
                    .or_default()
                    .insert(rule.selector.clone(), entities);
            }

            handled_roots.push(*root_entity);
        }
    }
    
    state.build(assets)
}

/// Select all entities using the given [`Selector`](crate::selector::Selector).
///
/// If no [`Children`] is supplied, then the selector is applied only on root entity.
fn select_entities(
    root: Entity,
    children: Option<&Children>,
    selector: &Selector,
    world: &World,
    css_query: &CssQueryParam,
    registry: &mut ComponentFilterRegistry,
) -> SmallVec<[Entity; 8]> {
    let mut parent_tree = selector.get_parent_tree();

    if parent_tree.is_empty() {
        return SmallVec::new();
    }

    let mut filter = children.map(|children| {
        // Include root, since style sheet may be applied on root too.
        std::iter::once(root)
            .chain(get_children_recursively(children, &css_query.children).into_iter())
            .collect()
    });

    loop {
        // TODO: Rework this to use a index to avoid recreating parent_tree every time the systems runs.
        // This is has little to no impact on performance, since this system doesn't runs often.
        let node = parent_tree.remove(0);

        let entities = select_entities_node(node, world, css_query, registry, filter.clone());

        if parent_tree.is_empty() {
            break entities;
        } else {
            let children = entities
                .into_iter()
                .filter_map(|e| css_query.children.get(e).ok())
                .flat_map(|(_e, ch)| get_children_recursively(ch, &css_query.children))
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
    filter: Option<SmallVec<[Entity; 8]>>,
) -> SmallVec<[Entity; 8]> {
    let fold_fn = |filter: Option<SmallVec<[Entity; 8]>>, element: &SelectorElement|
    -> Option<SmallVec<[Entity; 8]>> {
        let result = match element 
        {
            SelectorElement::Name(name) =>
            {
                get_entities_with(name.as_str(), &css_query.names, filter)
            },

            SelectorElement::Class(class) =>
            {
                get_entities_with(class.as_str(), &css_query.classes, filter)
            },

            #[cfg(feature = "pseudo_class")]
            SelectorElement::PseudoClass(class) =>
            {
                get_entities_with_pseudo_class(class.as_str(), &css_query.pseudo_classes, filter)
            },

            #[cfg(feature = "pseudo_prop")]
            SelectorElement::PseudoProp(prop) =>
            {
                todo!("Implement PseudoProperty selection")
            },

            SelectorElement::Component(component) =>
            {
                get_entities_with_component(component.as_str(), world, registry, filter)
            },

            // All child elements are filtered by [`get_parent_tree`](Selector::get_parent_tree)
            SelectorElement::Child =>
            {
                unreachable!()
            },
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
    filter: Option<SmallVec<[Entity; 8]>>,
) -> SmallVec<[Entity; 8]> {
    let mut result: SmallVec<[Entity; 8]> = Default::default();

    for (entity, action) in query.interaction.iter()
    {
        match (name, *action)
        {
            ("hover", Interaction::Hovered) => trace!("Entity[{entity:?}]:hover"),
            ("click", Interaction::Clicked) => trace!("Entity[{entity:?}]:click"),
            _ => continue,
        };

        if let Some(f) = &filter
        {
            if !f.contains(&entity)
            {
                trace!("Entity {entity:?} discarded by filter");
                continue;
            }
        }

        result.push(entity);
    }

    result
}

/// Utility function to filter any entities by using a component with implements [`MatchSelectorElement`]
fn get_entities_with<T>(
    name: &str,
    query: &Query<(Entity, &'static T)>,
    filter: Option<SmallVec<[Entity; 8]>>,
) -> SmallVec<[Entity; 8]>
where
    T: Component + MatchSelectorElement,
{
    query
        .iter()
        .filter_map(|(e, rhs)| 
            if rhs.matches(name) { Some(e) }
            else { None }
        )
        .filter(|e|
            if let Some(filter) = &filter { filter.contains(e) }
            else { true }
        )
        .collect()
}

/// Filters entities which have the components specified on selector, like "a" or "button".
///
/// The component must be registered on [`ComponentFilterRegistry`]
fn get_entities_with_component(
    name: &str,
    world: &World,
    components: &mut ComponentFilterRegistry,
    filter: Option<SmallVec<[Entity; 8]>>,
) -> SmallVec<[Entity; 8]> {
    if let Some(query) = components.0.get_mut(name)
    {
        if let Some(filter) = filter
        {
            query
                .filter(world)
                .into_iter()
                .filter(|e| filter.contains(e))
                .collect()
        }
        else
        {
            query.filter(world)
        }
    }
    else
    {
        error!("Unregistered component selector {}", name);
        SmallVec::new()
    }
}

fn get_children_recursively(
    children: &Children,
    query_childs: &query::QueryEntityChildren,
) -> SmallVec<[Entity; 8]> {
    children
        .iter()
        .flat_map(|&e|
            std::iter::once(e).chain(
                query_childs
                    .get(e)
                    .map_or(SmallVec::new(), |(_c, gc)| get_children_recursively(gc, query_childs)),
            )
        )
        .collect()
}

/// Auto reapply style sheets when hot reloading is enabled
pub(crate) fn hot_reload_style_sheets(
    mut assets_events: EventReader<AssetEvent<StyleSheetAsset>>,
    mut q_sheets: Query<&mut StyleSheet>,
) {
    for evt in assets_events.iter()
    {
        if let AssetEvent::Modified { handle } = evt
        {
            q_sheets
                .iter_mut()
                .filter(|sheet| sheet.handle() == handle)
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
