use super::{Property, PropertyValues};
use crate::prelude::BevyCssError;

/// Impls for `bevy_ui` [`Style`] component
pub mod style;

use bevy::{ecs::query::QueryItem, prelude::*};

/// Applies the `background-color` property on [`BackgroundColor`] component of matched entities.
#[derive(Default)]
pub(crate) struct BackgroundColorProperty;

impl Property for BackgroundColorProperty {
    type Cache = Color;
    type Components = (
        Option<&'static mut BackgroundColor>,
        Option<&'static mut UiImage>,
    );
    type Filters = ();

    fn name() -> &'static str {
        "background-color"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        if let Some(color) = values.color() {
            Ok(color)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        (bg, img): QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        if let Some(mut bg) = bg {
            *bg = BackgroundColor(*cache);
        }

        if let Some(mut img) = img {
            img.color = *cache;
        }
    }
}

/// Applies the `border-radius` property on [`BorderRadius`] component of matched entities.
#[derive(Default)]
pub(crate) struct BorderRadiusProperty;

impl Property for BorderRadiusProperty {
    type Cache = BorderRadius;
    type Components = Entity;
    type Filters = With<BorderRadius>;

    fn name() -> &'static str {
        "border-radius"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        if let Some(border) = values.border_radius() {
            Ok(border)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        component: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        commands: &mut Commands,
    ) {
        commands.entity(component).insert(*cache);
    }
}

/// Applies the `border-color` property on [`BorderColor`] component of matched entities.
#[derive(Default)]
pub(crate) struct BorderColorProperty;

impl Property for BorderColorProperty {
    type Cache = Color;
    type Components = &'static mut BorderColor;
    type Filters = ();

    fn name() -> &'static str {
        "border-color"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        if let Some(color) = values.color() {
            Ok(color)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut component: QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        component.0 = *cache;
    }
}