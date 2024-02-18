use crate::character::equipment::UnequipItem;
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
pub struct PickupItems {
    pub character: Entity,
}
impl Command for PickupItems {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);

        let transform = *world.get::<GlobalTransform>(self.character).unwrap();
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
            PickupItem {
                item,
                character: self.character,
            }
            .apply(world);
        }
    }
}

#[derive(Debug)]
pub struct PickupItem {
    pub character: Entity,
    pub item: Entity,
}
impl Command for PickupItem {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        debug_assert!(!world.entity(self.item).contains::<Parent>());

        world.entity_mut(self.character).add_child(self.item);

        *world.get_mut::<Visibility>(self.item).unwrap() = Visibility::Hidden;
        world.entity_mut(self.item).insert(RigidBodyDisabled);
        world.get_mut::<Transform>(self.item).unwrap().translation = Vec3::ZERO;
    }
}

#[derive(Debug)]
pub struct DropItem {
    pub character: Entity,
    pub item: Entity,
}
impl Command for DropItem {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        debug_assert!(world
            .entity(self.item)
            .get::<Parent>()
            .is_some_and(|p| p.get() == self.character));

        UnequipItem {
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
