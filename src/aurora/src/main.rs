use helios::bevy::prelude::*;
use helios::character::loot::CharacterLoot;
use helios::character::player::Player;
use helios::character::{CharacterBundle, CharacterName};
use helios::item::{
    ArmorBundle, ArmorDefense, ItemName, WeaponBundle, WeaponDamage, WeaponRange, WorldItem,
};
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
                ..default()
            },
            texture: asset_server.load("Player.png"),
            ..default()
        },
        Player,
    ));

    commands.spawn(CharacterBundle {
        name: CharacterName("Enemy1".to_string()),
        transform: Transform::from_xyz(400.0, 100.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture: asset_server.load("Enemy.png"),
        ..default()
    });

    let chestplate = commands
        .spawn(ArmorBundle {
            name: ItemName("Chestplate".to_string()),
            defense: ArmorDefense(10),
            transform: Transform::from_xyz(150.0, -200.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            texture: asset_server.load("Armor.png"),
            visibility: Visibility::Hidden,
            ..default()
        })
        .id();

    commands.spawn(CharacterBundle {
        name: CharacterName("Enemy2".to_string()),
        transform: Transform::from_xyz(400.0, -100.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture: asset_server.load("Enemy.png"),
        loot: CharacterLoot::Fixed(vec![chestplate]),
        ..default()
    });

    commands.spawn((
        WeaponBundle {
            name: ItemName("Claymore".to_string()),
            damage: WeaponDamage(10),
            range: WeaponRange(200.0),
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            texture: asset_server.load("Sword.png"),
            ..default()
        },
        WorldItem,
    ));
}
