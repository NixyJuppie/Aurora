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
pub struct GameCameraTarget {
    pub offset: Vec3,
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<GameCamera>, Without<GameCameraTarget>)>,
    target: Query<(&Transform, &GameCameraTarget), Without<GameCamera>>,
) {
    let mut camera_transform = camera.single_mut();
    let (target_transform, target) = target.single();

    camera_transform.translation = target_transform.translation + target.offset;
    camera_transform.look_at(target_transform.translation, Vec3::Y);
}
