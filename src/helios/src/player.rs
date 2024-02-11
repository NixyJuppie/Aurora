use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player;

fn move_player(mut player: Query<&mut Transform, With<Player>>, keys: Res<Input<KeyCode>>) {
    const SPEED: f32 = 10.0;
    let mut movement = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        movement.x += 1.0;
    }

    if movement != Vec3::ZERO {
        player.single_mut().translation += movement * SPEED;
    }
}
