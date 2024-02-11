use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct GameCamera;
#[derive(Component)]
pub struct GameCameraTarget {
    pub offset: Vec3,
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<GameCamera>, Without<GameCameraTarget>)>,
    target: Query<(&Transform, &GameCameraTarget), Without<GameCamera>>,
) {
    let mut camera = camera.single_mut();
    let target = target.single();

    camera.translation = target.0.translation + target.1.offset;
}
