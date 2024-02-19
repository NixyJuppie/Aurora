use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component, Default, Debug)]
pub struct GameCamera;

#[derive(Component, Default, Debug)]
pub struct GameCameraTarget;

fn update_camera(
    mut camera: Query<&mut Transform, (With<GameCamera>, Without<GameCameraTarget>)>,
    target: Query<&Transform, (With<GameCameraTarget>, Without<GameCamera>)>,
) {
    const UP_DISTANCE: f32 = 5.0;
    const BACK_DISTANCE: f32 = 10.0;

    let mut camera_transform = camera.single_mut();
    let Ok(target_transform) = target.get_single() else {
        return;
    };

    camera_transform.translation = target_transform.translation
        + (target_transform.back() * BACK_DISTANCE)
        + (target_transform.up() * UP_DISTANCE);
    camera_transform.look_at(target_transform.translation, Vec3::Y);
}

pub struct FollowTargetCommand(pub Entity);
impl Command for FollowTargetCommand {
    fn apply(self, world: &mut World) {
        for player in world
            .query_filtered::<Entity, With<GameCameraTarget>>()
            .iter(world)
            .collect::<Vec<Entity>>()
        {
            world.entity_mut(player).remove::<GameCameraTarget>();
        }

        world.entity_mut(self.0).insert(GameCameraTarget);
    }
}
