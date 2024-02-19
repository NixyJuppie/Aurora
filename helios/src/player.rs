use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::math::DEFAULT_EPSILON;

use crate::character::attack::AttackCommand;
use crate::character::inventory::PickupItemsCommand;
use crate::character::AttackCooldown;
use crate::input::GameplayInput;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
        app.add_systems(Update, rotate_player);
        app.add_systems(Update, pickup_items);
        app.add_systems(Update, jump);
        app.add_systems(Update, attack);
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

pub struct ImpersonateCommand(pub Entity);
impl Command for ImpersonateCommand {
    fn apply(self, world: &mut World) {
        for player in world
            .query_filtered::<Entity, With<Player>>()
            .iter(world)
            .collect::<Vec<Entity>>()
        {
            world.entity_mut(player).remove::<Player>();
        }

        world.entity_mut(self.0).insert(Player);
    }
}

fn move_player(
    mut players: Query<(&mut KinematicCharacterController, &Transform), With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Some(movement) = input.movement else {
        return;
    };

    const METERS_PER_SECOND: f32 = 10.0;
    for (mut controller, transform) in players.iter_mut() {
        let direction = (transform.forward() * movement.y) + (transform.right() * movement.x);
        controller.translation = Some(direction * METERS_PER_SECOND * time.delta_seconds());
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

fn jump(
    mut players: Query<(&mut Velocity, &KinematicCharacterController), With<Player>>,
    input: Res<GameplayInput>,
) {
    if !input.jump {
        return;
    }

    const JUMP_VELOCITY: f32 = 7.5;
    for (mut velocity, controller) in players.iter_mut() {
        if velocity.linvel.y.abs() < DEFAULT_EPSILON {
            velocity.linvel += controller.up * JUMP_VELOCITY;
        }
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
        commands.add(AttackCommand(player))
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
        commands.add(PickupItemsCommand(player))
    }
}
