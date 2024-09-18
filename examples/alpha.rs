use bevy::{
    ecs::query::QueryItem,
    prelude::*,
};
use tomt_bevycss::prelude::*;

fn main(
    // no args
) {
    // Whenever an StyleSheet is loaded, it'll be applied automatically
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(BevyCssPlugin::with_hot_reload())
        .add_systems(Startup, setup);

    app.register_property::<AlphaProperty>();
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle
        {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(StyleSheet::new(asset_server.load("sheets/alpha.css")))
        .with_children(|parent|
        {
            // bevy logo (image)
            parent.spawn(ImageBundle
            {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                ..default()
            })
            .insert(Class::new("logo"));

            // box
            parent.spawn(NodeBundle::default()).insert(Class::new("box"));
        });
}

#[derive(Default)]
pub(crate) struct AlphaProperty;

impl Property
for AlphaProperty
{
    // This is the cached value to be used when applying the property value.
    // It is evaluated only on the first time and futures values are cached for performance reasons.
    type Cache = f32;

    // Which components we need when applying the cache. It is the same as using bevy ecs Query.
    type Components = (
        Option<&'static mut BackgroundColor>,
        Option<&'static mut UiImage>,
    );

    // If this property can be set only when there is another property, we may filter there.
    // It's not recommended to use only With<> and Without<>.
    type Filters = ();

    fn name(
        // no args
    ) -> &'static str {
        // The name of property. prefer kebab-case for consistency.
        "alpha"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        // PropertyValues::f32 tries to parse property value into a numeric value
        match values.f32()
        {
            Some(value) => Ok(value),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string())),
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        (bg, img): QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        if let Some(mut bg) = bg {
            bg.0.set_alpha(*cache);
        } else if let Some(mut img) = img {
            img.color.set_alpha(*cache);
        }
    }
}
