use helios::bevy::prelude::*;
use helios::bevy_rapier3d::prelude::*;

use helios::camera::{GameCamera, GameCameraTarget};
use helios::character::attributes::{CharacterAttribute, Health};
use helios::character::bundles::CharacterBundle;
use helios::character::equipment::{CharacterEquipment, Weapon};
use helios::character::experience::CharacterLevel;
use helios::character::inventory::CharacterLoot;
use helios::character::CharacterName;
use helios::item::armor::ArmorProtection;
use helios::item::bundles::{ArmorBundle, WeaponBundle};
use helios::item::weapon::{DamageType, WeaponDamage, WeaponRange};
use helios::item::{ItemEquipmentSlot, ItemName};
use helios::player::Player;
use helios::{HeliosCollision, HeliosDebugPlugins, HeliosPlugins};

const GAME_NAME: &str = "Aurora";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((HeliosPlugins, HeliosDebugPlugins))
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        CharacterBundle {
            name: CharacterName("Player".to_string()),
            collider: Collider::capsule_y(0.5, 0.5),
            mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
            material: materials.add(Color::GOLD),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        LockedAxes::ROTATION_LOCKED,
        GameCameraTarget,
        Player,
    ));

    let enemy_sword = commands
        .spawn((
            WeaponBundle {
                name: ItemName("Longsword".to_string()),
                damage: WeaponDamage {
                    damage: 10,
                    damage_type: DamageType::Physical,
                },
                range: WeaponRange(3.0),
                collider: Collider::cuboid(0.2, 0.1, 0.5),
                mesh: meshes.add(Cuboid::new(0.4, 0.2, 1.0)),
                material: materials.add(Color::RED),
                visibility: Visibility::Hidden,
                ..default()
            },
            RigidBodyDisabled,
        ))
        .id();
    commands
        .spawn((
            CharacterBundle {
                name: CharacterName("Enemy".to_string()),
                level: CharacterLevel(30),
                health: CharacterAttribute::<Health>::new(20),
                collider: Collider::capsule_y(0.5, 0.5),
                mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
                material: materials.add(Color::RED),
                transform: Transform::from_xyz(-2.0, 2.0, -5.0)
                    .with_rotation(Quat::from_rotation_y(f32::to_radians(-135.0))),
                loot: CharacterLoot::Inventory,
                weapon: CharacterEquipment::<Weapon>::new(Some(enemy_sword)),
                ..default()
            },
            LockedAxes::ROTATION_LOCKED,
        ))
        .add_child(enemy_sword);

    commands.spawn(WeaponBundle {
        name: ItemName("Sword".to_string()),
        damage: WeaponDamage {
            damage: 10,
            damage_type: DamageType::Physical,
        },
        range: WeaponRange(2.0),
        collider: Collider::cuboid(0.2, 0.1, 0.5),
        mesh: meshes.add(Cuboid::new(0.4, 0.2, 1.0)),
        material: materials.add(Color::DARK_GRAY),
        transform: Transform::from_xyz(5.0, 5.0, -2.0),
        ..default()
    });

    commands.spawn(ArmorBundle {
        name: ItemName("Chainmail".to_string()),
        protection: ArmorProtection { physical: 5 },
        collider: Collider::cuboid(0.5, 0.1, 0.5),
        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(7.0, 5.0, -2.0),
        ..default()
    });

    commands.spawn(ArmorBundle {
        slot: ItemEquipmentSlot::Helmet,
        name: ItemName("Crown".to_string()),
        protection: ArmorProtection { physical: 4 },
        collider: Collider::cuboid(0.3, 0.1, 0.3),
        mesh: meshes.add(Cuboid::new(0.6, 0.2, 0.6)),
        material: materials.add(Color::GOLD),
        transform: Transform::from_xyz(6.0, 5.0, -1.0),
        ..default()
    });

    spawn_world(commands, materials, meshes);
}

fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -1.0, -0.33), Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle::default(), GameCamera));

    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(50.0, 0.02, 50.0)),
            material: materials.add(Color::GRAY),
            ..default()
        },
        Collider::cuboid(25.0, 0.01, 25.0),
        HeliosCollision::world_groups(),
    ));

    // slope
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(7.0, 0.75, 0.0)
                .with_rotation(Quat::from_rotation_x(f32::to_radians(20.0))),
            mesh: meshes.add(Cuboid::new(6.0, 0.5, 6.0)),
            material: materials.add(Color::GREEN),
            ..default()
        },
        Collider::cuboid(3.0, 0.25, 3.0),
        HeliosCollision::world_groups(),
    ));

    // step1
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-5.0, 0.2, 0.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.3, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.15, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step2
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-7.0, 0.55, 0.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step3
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-7.0, 0.9, -2.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step4
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-7.0, 1.25, -4.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step5
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-9.0, 1.6, -4.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step6
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-9.0, 1.95, -2.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));

    // step7
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-9.0, 2.3, 0.0),
            mesh: meshes.add(Cuboid::new(2.0, 0.6, 2.0)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
        HeliosCollision::world_groups(),
    ));
}
