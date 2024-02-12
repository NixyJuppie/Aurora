use crate::input::InGameInput;
use crate::schedule::InGameSet;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.in_set(InGameSet::EntityUpdate));
    }
}

#[derive(Component)]
pub struct Player;

fn move_player(mut player: Query<&mut Transform, With<Player>>, input: Res<InGameInput>) {
    const SPEED: f32 = 5.0;

    if input.movement != Vec2::ZERO {
        player.single_mut().translation +=
            Vec3::new(input.movement.x, input.movement.y, 0.0) * SPEED;
    }
}
