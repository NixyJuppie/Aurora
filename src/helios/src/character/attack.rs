use crate::character::attributes::{CharacterAttribute, Health};
use crate::character::equipment::{CharacterEquipment, Chest, EquipmentSlot, Helmet, Weapon};
use crate::character::AttackCooldown;
use crate::item::damage::{ArmorProtection, DamageType, WeaponDamage};
use crate::item::WeaponRange;
use crate::HeliosCollision;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug)]
pub struct AttackCommand {
    pub attacker: Entity,
}
impl Command for AttackCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        let Some(weapon) = world
            .get::<CharacterEquipment<Weapon>>(self.attacker)
            .unwrap()
            .entity
        else {
            warn!("Cannot attack without weapon equipped");
            return;
        };

        let mut cooldown = world.get_mut::<AttackCooldown>(self.attacker).unwrap();
        if !cooldown.0.finished() {
            warn!("Cannot attack during cooldown");
            return;
        }
        cooldown.0.reset();

        let range = world.get::<WeaponRange>(weapon).unwrap();
        let damage = world.get::<WeaponDamage>(weapon).unwrap();
        let mut cone_transform = *world.get::<Transform>(self.attacker).unwrap();
        cone_transform.rotate_x(f32::to_radians(-90.0));

        let mut commands = vec![];
        world
            .get_resource::<RapierContext>()
            .unwrap()
            .intersections_with_shape(
                cone_transform.translation,
                cone_transform.rotation,
                &Collider::cone(range.0 / 2.0, range.0),
                QueryFilter::new().groups(CollisionGroups::new(
                    HeliosCollision::CHARACTER.into(),
                    HeliosCollision::CHARACTER.into(),
                )),
                |target| {
                    if target != self.attacker {
                        commands.push(DealDamageCommand {
                            target,
                            damage: damage.damage,
                            damage_type: damage.damage_type,
                        });
                    }
                    true
                },
            );

        for command in commands {
            command.apply(world);
        }
    }
}

#[derive(Debug)]
pub struct DealDamageCommand {
    pub damage: u32,
    pub damage_type: DamageType,
    pub target: Entity,
}

impl Command for DealDamageCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

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

fn get_protection<S: EquipmentSlot + 'static>(
    world: &World,
    target: Entity,
    damage_type: DamageType,
) -> u32 {
    world
        .get::<CharacterEquipment<S>>(target)
        .and_then(|h| h.entity)
        .and_then(|h| world.get::<ArmorProtection>(h).map(|p| p.get(damage_type)))
        .unwrap_or(0)
}

#[derive(Debug)]
pub struct KillCommand {
    pub target: Entity,
}
impl Command for KillCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        world.entity_mut(self.target).despawn_recursive();
    }
}
