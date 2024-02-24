use helios::bevy::prelude::*;
use helios::bevy_rapier3d::prelude::*;

use helios::bevy::utils::smallvec::smallvec;
use helios::camera::{GameCamera, GameCameraTarget};
use helios::character::ai::EnemyRange;
use helios::character::attributes::{CharacterAttribute, Health};
use helios::character::bundles::CharacterBundle;
use helios::character::equipment::{CharacterEquipment, Weapon};
use helios::character::experience::CharacterLevel;
use helios::character::inventory::CharacterLoot;
use helios::character::CharacterName;
use helios::item::armor::ArmorProtection;
use helios::item::bundles::{ArmorBundle, ItemContainerBundle, WeaponBundle};
use helios::item::container::ItemContainerName;
use helios::item::weapon::{Damage, DamageType, WeaponDamage, WeaponRange};
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

    spawn_enemy(
        &mut commands,
        &mut materials,
        &mut meshes,
        Transform::from_xyz(-10.0, 2.0, -20.0),
        Color::ORANGE_RED,
        10,
        5.0,
    );

    spawn_enemy(
        &mut commands,
        &mut materials,
        &mut meshes,
        Transform::from_xyz(10.0, 2.0, -20.0),
        Color::RED,
        25,
        3.0,
    );

    commands.spawn(WeaponBundle {
        name: ItemName("Fire dagger".to_string()),
        damage: WeaponDamage(smallvec![
            Damage {
                damage: 10,
                damage_type: DamageType::Physical,
            },
            Damage {
                damage: 5,
                damage_type: DamageType::Fire,
            }
        ]),
        range: WeaponRange(1.5),
        collider: Collider::cuboid(0.2, 0.1, 0.5),
        mesh: meshes.add(Cuboid::new(0.4, 0.2, 1.0)),
        material: materials.add(Color::DARK_GRAY),
        transform: Transform::from_xyz(6.0, 5.0, -2.0),
        ..default()
    });

    let chainmail = commands
        .spawn((
            ArmorBundle {
                name: ItemName("Chainmail".to_string()),
                protection: ArmorProtection {
                    physical: 5,
                    ..default()
                },
                collider: Collider::cuboid(0.5, 0.1, 0.5),
                mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                material: materials.add(Color::BLUE),
                visibility: Visibility::Hidden,
                ..default()
            },
            RigidBodyDisabled,
        ))
        .id();

    let crown = commands
        .spawn((
            ArmorBundle {
                slot: ItemEquipmentSlot::Helmet,
                name: ItemName("Crown".to_string()),
                protection: ArmorProtection {
                    physical: 4,
                    ..default()
                },
                collider: Collider::cuboid(0.3, 0.1, 0.3),
                mesh: meshes.add(Cuboid::new(0.6, 0.2, 0.6)),
                material: materials.add(Color::GOLD),
                visibility: Visibility::Hidden,
                ..default()
            },
            RigidBodyDisabled,
        ))
        .id();

    commands
        .spawn(ItemContainerBundle {
            name: ItemContainerName("Chest".to_string()),
            transform: Transform::from_xyz(13.0, 1.0, 2.0),
            mesh: meshes.add(Cuboid::new(3.0, 1.5, 2.0)),
            collider: Collider::cuboid(1.5, 0.75, 1.0),
            material: materials.add(Color::rgb_u8(139, 69, 19)),
            ..default()
        })
        .add_child(chainmail)
        .add_child(crown);

    spawn_world(&mut commands, &mut materials, &mut meshes);
}

fn spawn_enemy(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    transform: Transform,
    color: Color,
    weapon_damage: u32,
    weapon_range: f32,
) {
    let weapon_name = if weapon_range > 4.0 {
        "Longsword"
    } else {
        "Sword"
    };
    let enemy_sword = commands
        .spawn((
            WeaponBundle {
                name: ItemName(weapon_name.to_string()),
                damage: WeaponDamage(smallvec![Damage {
                    damage: weapon_damage,
                    damage_type: DamageType::Physical,
                }]),
                range: WeaponRange(weapon_range),
                collider: Collider::cuboid(0.2, 0.1, 0.5),
                mesh: meshes.add(Cuboid::new(0.4, 0.2, 1.0)),
                material: materials.add(color),
                visibility: Visibility::Hidden,
                ..default()
            },
            RigidBodyDisabled,
        ))
        .id();
    commands
        .spawn((
            EnemyRange(17.5),
            CharacterBundle {
                transform,
                name: CharacterName("Enemy".to_string()),
                level: CharacterLevel(30),
                health: CharacterAttribute::<Health>::new(20),
                collider: Collider::capsule_y(0.5, 0.5),
                mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
                material: materials.add(color),
                loot: CharacterLoot::Inventory,
                weapon: CharacterEquipment::<Weapon>::new(Some(enemy_sword)),
                ..default()
            },
            LockedAxes::ROTATION_LOCKED,
        ))
        .add_child(enemy_sword);
}

fn spawn_world(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
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
            mesh: meshes.add(Cuboid::new(100.0, 0.02, 100.0)),
            material: materials.add(Color::GRAY),
            ..default()
        },
        Collider::cuboid(50.0, 0.01, 50.0),
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

    // stairs
    let mesh = meshes.add(Cuboid::new(2.0, 0.3, 2.0));
    let material = materials.add(Color::BLACK);
    for i in 0..10 {
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    -5.0 - (2.0 * i as f32),
                    0.2 + (0.35 * i as f32),
                    5.0,
                ),
                ..default()
            },
            Collider::cuboid(1.0, 0.15, 1.0),
            HeliosCollision::world_groups(),
        ));
    }

    for i in 0..10 {
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    -25.0,
                    3.7 + (0.35 * i as f32),
                    5.0 - (2.0 * i as f32),
                ),
                ..default()
            },
            Collider::cuboid(1.0, 0.15, 1.0),
            HeliosCollision::world_groups(),
        ));
    }
}
