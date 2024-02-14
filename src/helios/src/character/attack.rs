use crate::character::attributes::CharacterHealth;
use crate::character::inventory::{CharacterArmor, CharacterWeapon};
use crate::character::CharacterLookDirection;
use crate::item::{ArmorDefense, WeaponDamage, WeaponRange};
use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct Attack {
    pub character: Entity,
}
impl Command for Attack {
    fn apply(self, world: &mut World) {
        const ATTACK_ANGLE: f32 = 90.0;

        let attacker = world.entity(self.character);
        let Some(weapon) = attacker.get::<CharacterWeapon>().and_then(|w| w.0) else {
            warn!("Cannot attack without equipped weapon");
            return;
        };
        let attacker_translation = attacker.get::<GlobalTransform>().unwrap().translation();
        let attacker_direction = attacker.get::<CharacterLookDirection>().unwrap().0;
        let damage = world.entity(weapon).get::<WeaponDamage>().unwrap().0;
        let range = world.entity(weapon).get::<WeaponRange>().unwrap().0;

        let hit_entities = world.query_filtered::<(&GlobalTransform, Entity), (With<CharacterHealth>, With<CharacterArmor>)>()
            .iter(world).filter(|(target_transform, _)| {
            let direction = target_transform.translation() - attacker_translation;
                if direction.length() <= range {
                    let angle = direction.angle_between(Vec3::new(
                        attacker_direction.x,
                        attacker_direction.y,
                        direction.z,
                    ));

                    return angle.to_degrees().abs() <= ATTACK_ANGLE;
                }
            false
        }).map(|e| e.1).collect::<Vec<Entity>>();

        for target in hit_entities {
            let armor = world.get::<CharacterArmor>(target).unwrap().0;

            let defense = armor
                .and_then(|a| world.get::<ArmorDefense>(a).map(|d| d.0))
                .unwrap_or(0);
            let final_damage = damage.saturating_sub(defense);
            info!(
                "Attack damage: {} (Weapon) - {} (Armor) = {}",
                damage, defense, final_damage
            );

            if final_damage != 0 {
                let mut health = world.get_mut::<CharacterHealth>(target).unwrap();
                health.0.current = health.0.current.saturating_sub(final_damage);
                info!(
                    "Hit character for {}, remaining health: {}",
                    final_damage, health.0.current
                );

                if health.0.current == 0 {
                    world.entity_mut(target).despawn_recursive();
                }
            }
        }
    }
}
