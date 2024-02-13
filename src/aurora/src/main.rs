use helios::bevy::prelude::*;
use helios::character::player::Player;
use helios::character::{CharacterBundle, CharacterName};
use helios::item::{ItemBundle, ItemName, WorldItem};
use helios::HeliosPlugins;

const GAME_NAME: &str = "Aurora";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_NAME.to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(HeliosPlugins)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        CharacterBundle {
            name: CharacterName("Player".to_string()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                color: Color::GOLD,
                ..default()
            },
            texture: asset_server.load("Player.png"),
            ..default()
        },
        Player,
    ));

    commands.spawn((
        ItemBundle {
            name: ItemName("Sword".to_string()),
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::GOLD,
                ..default()
            },
            texture: asset_server.load("Sword.png"),
            ..default()
        },
        WorldItem,
    ));
}
