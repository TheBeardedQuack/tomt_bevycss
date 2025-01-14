use bevy::{
    prelude::*,
    ui::FocusPolicy,
};
use tomt_bevycss::prelude::*;

#[derive(Debug, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct Title;

fn main(
    // no args
) {
    App::new()
        .add_plugins(DefaultPlugins)
        // Whenever an StyleSheet is loaded, it'll be applied automatically
        .add_plugins(BevyCssPlugin::with_hot_reload())
        .add_systems(Startup, setup)
        .add_systems(Update, change_theme)
        .register_component_selector::<Title>("title")
        .run();
}

#[derive(Resource)]
struct Themes
{
    pub root: Entity,
    pub dark: Handle<StyleSheetAsset>,
    pub light: Handle<StyleSheetAsset>,
}

fn change_theme(
    themes: Res<Themes>,
    mut styles_query: Query<&mut StyleSheet>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query
    {
        match *interaction
        {
            Interaction::None => info!("Button left"),
            Interaction::Hovered => info!("Button hovered"),
            Interaction::Pressed => {
                info!("Button clicked");
                if let Ok(mut sheet) = styles_query.get_mut(themes.root)
                {
                    let new_sheet = match sheet.handle() == &themes.light
                    {
                        true => &themes.dark,
                        false => &themes.light,
                    };

                    sheet.set(new_sheet.clone());
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let dark = asset_server.load("sheets/dark_theme.css");
    let light = asset_server.load("sheets/light_theme.css");

    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    let root = commands.spawn(NodeBundle
        {
            style: Style
            {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert((
            Name::new("ui-root"),
            StyleSheet::new(dark.clone())
        ))
        .with_children(|parent|
        {
            // left vertical fill (border)
            parent.spawn(NodeBundle
                {
                    style: Style
                    {
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .insert(Name::new("left-border"))
                .with_children(|parent|
                {
                    // left vertical fill (content)
                    parent.spawn(NodeBundle
                        {
                            style: Style
                            {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(Name::new("left-bf"))
                        .with_children(|parent|
                        {
                            // text
                            parent.spawn(TextBundle::from_section("Text Example", TextStyle
                                    {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    })
                                    .with_style(Style
                                    {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    })
                                )
                                .insert(Name::new("left-text"));
                        });
                });

            // right vertical fill
            parent.spawn(NodeBundle
                {
                    style: Style
                    {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(Name::new("right-border"))
                .with_children(|parent|
                {
                    // Title
                    parent.spawn(TextBundle::from_section("Scrolling list", TextStyle
                            {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            })
                            .with_style(Style
                            {
                                height: Val::Px(25.0),
                                margin: UiRect
                                {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..default()
                                },
                                ..default()
                            })
                        )
                        .insert((
                            Name::new("right-bg"),
                            Title,
                        ));

                    // List with hidden overflow
                    parent.spawn(NodeBundle
                        {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_self: AlignSelf::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                overflow: Overflow {
                                    x: OverflowAxis::Clip,
                                    y: OverflowAxis::Clip,
                                },
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .insert(Name::new("right-list"))
                        .with_children(|parent|
                        {
                            // Moving panel
                            parent.spawn(NodeBundle
                                {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        flex_grow: 1.0,
                                        ..default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(Name::new("right-moving-panel"))
                                .with_children(|parent|
                                {
                                    // List items
                                    for i in 0..30
                                    {
                                        parent.spawn(TextBundle::from_section(format!("Item {i}"), TextStyle
                                                {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                })
                                                .with_style(Style
                                                {
                                                    flex_shrink: 0.,
                                                    width: Val::DEFAULT,
                                                    height: Val::Px(20.0),
                                                    margin: UiRect
                                                    {
                                                        left: Val::Auto,
                                                        right: Val::Auto,
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                            )
                                            .insert((
                                                Name::new(format!("right-item-{}", i)),
                                                Class::new("big-text"),
                                            ));
                                    }
                                });
                        });
                });

            // render order test: reddest in the back, whitest in the front (flex center)
            parent.spawn(NodeBundle
                {
                    style: Style
                    {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .insert((
                    Name::new("mid-red-last"),
                    Class::new("blue-bg container"),
                ))
                .with_children(|parent|
                {
                    parent.spawn(NodeBundle
                        {
                            style: Style
                            {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..default()
                        })
                        .insert(Name::new("mid-red-last-but-one"))
                        .with_children(|parent|
                        {
                            parent.spawn(NodeBundle
                                {
                                    style: Style
                                    {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                                    ..default()
                                })
                                .insert(Name::new("mid-red-center"));

                            parent.spawn(NodeBundle
                                {
                                    style: Style
                                    {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                                    ..default()
                                })
                                .insert((
                                    Class::new("blue-bg"),
                                    Name::new("mid-red-top-but-one"),
                                ));

                            parent.spawn(NodeBundle
                                {
                                    style: Style
                                    {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                                    ..default()
                                })
                                .insert(Name::new("mid-red-top"));

                            // alpha test
                            parent.spawn(NodeBundle
                                {
                                    style: Style
                                    {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                    ..default()
                                })
                                .insert((
                                    Class::new("blue-bg"),
                                    Name::new("mid-red-alpha"),
                                ));
                        });
                });

            // bevy logo (flex center)
            parent.spawn(NodeBundle
                {
                    style: Style
                    {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .insert(Name::new("mid-bevy-logo-bg"))
                .with_children(|parent|
                {
                    // bevy logo (image)
                    parent.spawn(ImageBundle
                        {
                            style: Style
                            {
                                width: Val::Px(500.0),
                                height: Val::Auto,
                                ..default()
                            },
                            image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                            ..default()
                        })
                        .insert(Name::new("mid-bevy-logo-image"));
                });

            // absolute positioning
            parent.spawn(NodeBundle
                {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        left: Val::Px(210.0),
                        bottom: Val::Px(10.0),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..default()
                })
                .insert(Name::new("mid-blue-border"))
                .with_children(|parent|
                {
                    parent.spawn(NodeBundle
                        {
                            style: Style
                            {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            focus_policy: FocusPolicy::Pass,
                            background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                            ..default()
                        })
                        .insert(Name::new("mid-navy-blue-content"))
                        .with_children(|parent|
                        {
                            parent.spawn(ButtonBundle
                                {
                                    style: Style
                                    {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent|
                                {
                                    parent.spawn(TextBundle::from_section("Change Theme", TextStyle
                                        {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                        });
                });
        })
        .id();

    commands.insert_resource(Themes { root, dark, light })
}
