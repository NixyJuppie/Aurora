use helios::bevy::prelude::*;
use helios::camera::{GameCamera, GameCameraTarget};
use helios::character::{CharacterBundle, CharacterName};
use helios::player::Player;
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
    commands.spawn((Camera2dBundle::default(), GameCamera));
    commands.spawn((
        CharacterBundle {
            name: CharacterName("Player".to_string()),
            ..default()
        },
        SpriteBundle {
            texture: asset_server.load("Player.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                color: Color::GOLD,
                ..default()
            },
            ..default()
        },
        GameCameraTarget { offset: Vec3::ZERO },
        Player,
    ));
}
