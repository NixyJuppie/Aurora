use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::character::attack::AttackCommand;
use crate::character::inventory::PickupItems;
use crate::character::AttackCooldown;
use crate::input::GameplayInput;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
        app.add_systems(Update, rotate_player);
        app.add_systems(Update, pickup_items);
        app.add_systems(Update, attack);
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

fn move_player(
    mut players: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Some(movement) = input.movement else {
        return;
    };

    const METERS_PER_SECOND: f32 = 10.0;
    for mut player in players.iter_mut() {
        let movement =
            Vec3::new(movement.x, 0.0, -movement.y) * METERS_PER_SECOND * time.delta_seconds();
        player.translation = Some(movement);
    }
}

fn rotate_player(
    mut players: Query<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Some(rotation) = input.rotate else {
        return;
    };

    const DEGREES_PER_SECOND: f32 = 180.0;
    for mut player in players.iter_mut() {
        let rotation = -rotation * DEGREES_PER_SECOND * time.delta_seconds();
        player.rotate_y(rotation.to_radians());
    }
}

fn attack(
    mut commands: Commands,
    players: Query<(&AttackCooldown, Entity), With<Player>>,
    input: Res<GameplayInput>,
) {
    if !input.attack {
        return;
    }

    for (_, player) in players.iter().filter(|(c, _)| c.0.finished()) {
        commands.add(AttackCommand { attacker: player })
    }
}

fn pickup_items(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    input: Res<GameplayInput>,
) {
    if !input.pickup {
        return;
    }

    for player in players.iter() {
        commands.add(PickupItems { character: player })
    }
}
