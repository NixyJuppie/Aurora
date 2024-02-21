use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::character::attributes::{CharacterAttribute, Health};
use crate::character::equipment::{CharacterEquipment, Chest, EquipmentSlot, Helmet, Weapon};
use crate::character::experience::{CharacterExperience, CharacterLevel};
use crate::character::inventory::{CharacterLoot, DropItemCommand};
use crate::character::AttackCooldown;
use crate::item::armor::ArmorProtection;
use crate::item::weapon::{DamageType, WeaponDamage, WeaponRange};
use crate::HeliosCollision;

#[derive(Debug)]
pub struct AttackCommand(pub Entity);

impl Command for AttackCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        let Some(weapon) = world
            .get::<CharacterEquipment<Weapon>>(self.0)
            .unwrap()
            .entity
        else {
            warn!("Cannot attack without weapon equipped");
            return;
        };

        let mut cooldown = world.get_mut::<AttackCooldown>(self.0).unwrap();
        if !cooldown.0.finished() {
            warn!("Cannot attack during cooldown");
            return;
        }
        cooldown.0.reset();

        let radius = world.get::<WeaponRange>(weapon).unwrap().0 / 2.0;
        let damage = world.get::<WeaponDamage>(weapon).unwrap().clone();
        let transform = *world.get::<Transform>(self.0).unwrap();

        let mut targets = vec![];
        world
            .get_resource::<RapierContext>()
            .unwrap()
            .intersections_with_shape(
                transform.translation + (transform.forward() * radius),
                transform.rotation,
                &Collider::ball(radius),
                QueryFilter::new().groups(HeliosCollision::character_only_groups()),
                |target| {
                    if target != self.0 {
                        targets.push(target);
                    }
                    true
                },
            );

        for target in targets {
            DealDamageCommand {
                target,
                attacker: self.0,
                damage: damage.damage,
                damage_type: damage.damage_type,
            }
            .apply(world);
        }
    }
}

#[derive(Debug)]
pub struct DealDamageCommand {
    pub damage: u32,
    pub damage_type: DamageType,
    pub attacker: Entity,
    pub target: Entity,
}

impl Command for DealDamageCommand {
    fn apply(self, world: &mut World) {
        debug!("{:?}", self);

        let protection = get_protections(world, self.target, self.damage_type);
        let damage = self.damage.saturating_sub(protection);
        if damage > 0 {
            let mut health = world
                .get_mut::<CharacterAttribute<Health>>(self.target)
                .unwrap();
            health.current = health.current.saturating_sub(damage);
            info!(
                "Dealt {} damage to {:?}, remaining health is {}/{}",
                damage, self.target, health.current, health.max
            );

            if health.current == 0 {
                KillCommand {
                    killer: self.attacker,
                    target: self.target,
                }
                .apply(world);
            }
        }
    }
}

fn get_protections(world: &World, target: Entity, damage_type: DamageType) -> u32 {
    get_protection::<Helmet>(world, target, damage_type)
        + get_protection::<Chest>(world, target, damage_type)
}

fn get_protection<S: EquipmentSlot>(world: &World, target: Entity, damage_type: DamageType) -> u32 {
    world
        .get::<CharacterEquipment<S>>(target)
        .and_then(|h| h.entity)
        .and_then(|h| world.get::<ArmorProtection>(h).map(|p| p.get(damage_type)))
        .unwrap_or(0)
}

#[derive(Debug)]
pub struct KillCommand {
    pub killer: Entity,
    pub target: Entity,
}

impl Command for KillCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        let items = match world.get::<CharacterLoot>(self.target).unwrap() {
            CharacterLoot::None => vec![],
            CharacterLoot::Inventory => world
                .get::<Children>(self.target)
                .map(|c| c.iter().cloned().collect())
                .unwrap_or_default(),
            CharacterLoot::Fixed(items) => items.clone(),
        };
        for item in items {
            DropItemCommand {
                item,
                character: self.target,
            }
            .apply(world);
        }

        let level = world.get::<CharacterLevel>(self.target).unwrap();
        world.get_mut::<CharacterExperience>(self.killer).unwrap().0 += level.0 as u32 * 10;

        world.entity_mut(self.target).despawn_recursive();
    }
}
