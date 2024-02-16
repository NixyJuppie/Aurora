use helios::bevy::prelude::*;
use helios::bevy_rapier3d::geometry::Collider;
use helios::bevy_rapier3d::prelude::{LockedAxes, RigidBody};
use helios::camera::{GameCamera, GameCameraTarget};
use helios::character::player::PlayerBundle;
use helios::HeliosPlugins;

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
        .add_plugins(HeliosPlugins)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -1.0, -0.33), Vec3::Y),
        ..default()
    });
    commands.spawn((Camera3dBundle::default(), GameCamera));

    commands.spawn((
        PlayerBundle {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::capsule_y(0.5, 0.5),
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(Color::GOLD.into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        LockedAxes::ROTATION_LOCKED,
        GameCameraTarget {
            offset: Vec3::new(0.0, 4.0, 10.0),
        },
    ));

    spawn_world(commands, materials, meshes);
}

fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.0).into()),
            material: materials.add(Color::BEIGE.into()),
            ..default()
        },
        Collider::cuboid(10.0, 0.01, 10.0),
    ));
    // step1
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-5.0, 0.3, 0.0),
            mesh: meshes.add(shape::Box::new(2.0, 0.3, 2.0).into()),
            material: materials.add(Color::BLACK.into()),
            ..default()
        },
        Collider::cuboid(1.0, 0.15, 1.0),
    ));
    // step2
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-7.0, 0.6, 0.0),
            mesh: meshes.add(shape::Box::new(2.0, 0.6, 2.0).into()),
            material: materials.add(Color::BLACK.into()),
            ..default()
        },
        Collider::cuboid(1.0, 0.3, 1.0),
    ));
    // slope
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(7.0, 0.75, 0.0)
                .with_rotation(Quat::from_rotation_x(f32::to_radians(20.0))),
            mesh: meshes.add(shape::Box::new(6.0, 0.5, 6.0).into()),
            material: materials.add(Color::GREEN.into()),
            ..default()
        },
        Collider::cuboid(3.0, 0.25, 3.0),
    ));
}
