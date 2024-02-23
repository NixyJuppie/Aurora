use crate::character::equipment::UnequipItemCommand;
use crate::item::container::ItemContainerName;
use crate::HeliosCollision;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default, Debug)]
pub enum CharacterLoot {
    #[default]
    None,
    Inventory,
    Fixed(Vec<Entity>),
}

pub const PICKUP_RADIUS: f32 = 2.0;

#[derive(Debug)]
pub struct PickupItemsCommand(pub Entity);

impl Command for PickupItemsCommand {
    fn apply(self, world: &mut World) {
        debug!("{:?}", self);

        let transform = *world.get::<GlobalTransform>(self.0).unwrap();
        let mut items = vec![];
        world
            .get_resource::<RapierContext>()
            .unwrap()
            .intersections_with_shape(
                transform.translation(),
                Quat::default(),
                &Collider::capsule_x(PICKUP_RADIUS, PICKUP_RADIUS),
                QueryFilter::new().groups(HeliosCollision::item_only_groups()),
                |item| {
                    items.push(item);
                    true
                },
            );

        for item in items {
            PickupItemCommand {
                item,
                character: self.0,
            }
            .apply(world);
        }
    }
}

#[derive(Debug)]
pub struct PickupItemCommand {
    pub character: Entity,
    pub item: Entity,
}

impl Command for PickupItemCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        #[cfg(debug_assertions)]
        {
            // Item can only be picked up from world (no parent) or from container
            if world.entity(self.item).contains::<Parent>() {
                debug_assert!(world
                    .get::<Parent>(self.item)
                    .is_some_and(|p| world.entity(p.get()).contains::<ItemContainerName>()));
            }
        }

        world.entity_mut(self.character).add_child(self.item);

        *world.get_mut::<Visibility>(self.item).unwrap() = Visibility::Hidden;
        world.entity_mut(self.item).insert(RigidBodyDisabled);
        world.get_mut::<Transform>(self.item).unwrap().translation = Vec3::ZERO;
    }
}

#[derive(Debug)]
pub struct DropItemCommand {
    pub character: Entity,
    pub item: Entity,
}

impl Command for DropItemCommand {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        debug_assert!(world
            .entity(self.item)
            .get::<Parent>()
            .is_some_and(|p| p.get() == self.character));

        UnequipItemCommand {
            character: self.character,
            item: self.item,
        }
        .apply(world);

        world
            .entity_mut(self.character)
            .remove_children(&[self.item]);

        let target_translation = world
            .get::<GlobalTransform>(self.character)
            .unwrap()
            .translation();
        *world.get_mut::<Visibility>(self.item).unwrap() = Visibility::Inherited;
        world.entity_mut(self.item).remove::<RigidBodyDisabled>();
        world.get_mut::<Transform>(self.item).unwrap().translation = target_translation;
    }
}
