use crate::character::attack::AttackCommand;
use crate::character::equipment::{CharacterEquipment, Weapon};
use crate::character::AttackCooldown;
use crate::item::weapon::WeaponRange;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::parry::math::DEFAULT_EPSILON;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default, Debug)]
pub struct EnemyRange(pub f32);

pub fn follow_and_attack_player(
    mut commands: Commands,
    mut enemies: Query<
        (
            Entity,
            &mut KinematicCharacterController,
            &mut Transform,
            &mut Velocity,
            &EnemyRange,
            &AttackCooldown,
            &CharacterEquipment<Weapon>,
        ),
        Without<Player>,
    >,
    weapons: Query<&WeaponRange>,
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Some(player) = players.iter().next() else {
        return;
    };

    for (enemy, mut controller, mut transform, mut velocity, range, cooldown, weapon) in
        enemies.iter_mut()
    {
        if let Some(weapon_range) = weapon.entity.and_then(|w| weapons.get(w).ok()) {
            let difference = player.translation - transform.translation;
            let direction = difference.normalize();
            let distance = difference.length().abs();

            const METERS_PER_SECOND: f32 = 5.0;
            if distance >= weapon_range.0 / 2.0 && distance < range.0 {
                let look_target = Vec3::new(
                    player.translation.x,
                    transform.translation.y,
                    player.translation.z,
                );
                transform.look_at(look_target, Vec3::Y);
                controller.translation = Some(direction * METERS_PER_SECOND * time.delta_seconds());

                if distance <= weapon_range.0 * 2.0
                    && player.translation.y > transform.translation.y + (weapon_range.0 / 2.0)
                {
                    const JUMP_VELOCITY: f32 = 7.5;
                    if velocity.linvel.y.abs() < DEFAULT_EPSILON {
                        velocity.linvel += controller.up * JUMP_VELOCITY;
                    }
                }
            }

            if cooldown.0.finished() && distance <= weapon_range.0 {
                commands.add(AttackCommand(enemy));
            }
        }
    }
}
