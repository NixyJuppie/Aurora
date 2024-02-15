use crate::character::attributes::CharacterHealth;
use crate::character::damage::SpawnDamageIndicator;
use crate::character::inventory::{CharacterArmor, CharacterWeapon, DropItem};
use crate::character::loot::CharacterLoot;
use crate::character::CharacterLookDirection;
use crate::item::{ArmorDefense, WeaponDamage, WeaponRange};
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Attack {
    pub character: Entity,
}
impl Command for Attack {
    fn apply(self, world: &mut World) {
        const ATTACK_ANGLE: f32 = 90.0;

        info!("{:?}", self);
        let Some(weapon) = world
            .get::<CharacterWeapon>(self.character)
            .and_then(|w| w.0)
        else {
            warn!("Cannot attack without equipped weapon");
            return;
        };
        let attacker_translation = world
            .get::<GlobalTransform>(self.character)
            .unwrap()
            .translation();
        let attacker_direction = world
            .get::<CharacterLookDirection>(self.character)
            .unwrap()
            .0;
        let damage = world.entity(weapon).get::<WeaponDamage>().unwrap().0;
        let range = world.entity(weapon).get::<WeaponRange>().unwrap().0;

        let hit_entities = world.query_filtered::<(&GlobalTransform, Entity), (With<CharacterHealth>, With<CharacterArmor>)>()
            .iter(world)
            .filter(|(target_transform, _)| {
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
            })
            .map(|e| e.1)
            .collect::<Vec<Entity>>();

        for target in hit_entities {
            DealDamage { target, damage }.apply(world);
        }
    }
}

#[derive(Debug)]
pub struct DealDamage {
    pub target: Entity,
    pub damage: u32,
}
impl Command for DealDamage {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        let armor = world.get::<CharacterArmor>(self.target).unwrap().0;

        let defense = armor
            .and_then(|a| world.get::<ArmorDefense>(a).map(|d| d.0))
            .unwrap_or(0);
        let damage = self.damage.saturating_sub(defense);

        SpawnDamageIndicator {
            damage,
            target: self.target,
        }
        .apply(world);

        if damage != 0 {
            let mut health = world.get_mut::<CharacterHealth>(self.target).unwrap();
            health.0.current = health.0.current.saturating_sub(damage);
            info!(
                "Hit character {:?} for {}, remaining health is {}",
                self.target, damage, health.0.current
            );

            if health.0.current == 0 {
                Kill {
                    character: self.target,
                }
                .apply(world);
            }
        }
    }
}

#[derive(Debug)]
pub struct Kill {
    pub character: Entity,
}
impl Command for Kill {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        let items: Vec<Entity> = match world.get::<CharacterLoot>(self.character).unwrap() {
            CharacterLoot::None => vec![],
            CharacterLoot::Fixed(ref items) => items.clone(),
        };

        for item in items {
            DropItem {
                item,
                character: self.character,
            }
            .apply(world);
        }

        world.entity_mut(self.character).despawn_recursive();
    }
}
