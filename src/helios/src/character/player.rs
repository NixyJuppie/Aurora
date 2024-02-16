use crate::input::GameplayInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    // core
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    // mesh
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    // physics
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
}

fn move_player(
    mut players: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    const SPEED: f32 = 10.0;

    if let Some(movement) = input.movement {
        for mut player in players.iter_mut() {
            let movement = Vec3::new(movement.x, 0.0, -movement.y) * SPEED * time.delta_seconds();
            player.translation = Some(movement);
        }
    };
}
