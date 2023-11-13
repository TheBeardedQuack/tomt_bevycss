use bevy::prelude::*;
use tomt_bevycss::prelude::{
    BevyCssPlugin,
    Class,
    StyleSheet,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyCssPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            ..Default::default()
        })
        .insert(Name::new("root"))
        .insert(StyleSheet::new(asset_server.load("sheets/stress.css")))
        .with_children(|builder| {
            for _ in 0..10 {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Class::new("red"))
                    .with_children(|builder| {
                        for _ in 0..10 {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Class::new("green"))
                                .with_children(|builder| {
                                    for _ in 0..10 {
                                        builder
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            })
                                            .insert(Class::new("blue"))
                                            .with_children(|builder| {
                                                for _ in 0..10 {
                                                    builder
                                                        .spawn(NodeBundle {
                                                            ..Default::default()
                                                        })
                                                        .insert(Class::new("purple"));
                                                }
                                            });
                                    }
                                });
                        }
                    });
            }
        });
}
