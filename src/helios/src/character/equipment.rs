use crate::item::ItemEquipmentSlot;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component, Default, Debug)]
pub struct CharacterEquipment<S: EquipmentSlot> {
    slot_marker: PhantomData<S>,
    pub entity: Option<Entity>,
}

impl<S: EquipmentSlot> CharacterEquipment<S> {
    pub fn new(entity: Option<Entity>) -> Self {
        Self {
            entity,
            slot_marker: PhantomData,
        }
    }
}

pub trait EquipmentSlot: Send + Sync + 'static {}
impl EquipmentSlot for Chest {}
impl EquipmentSlot for Helmet {}
impl EquipmentSlot for Weapon {}

#[derive(Default, Debug)]
pub struct Chest;

#[derive(Default, Debug)]
pub struct Helmet;

#[derive(Default, Debug)]
pub struct Weapon;

#[derive(Debug)]
pub struct EquipItem {
    pub character: Entity,
    pub item: Entity,
}
impl Command for EquipItem {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        debug_assert!(world
            .get::<Parent>(self.item)
            .is_some_and(|p| p.get() == self.character));

        let Some(slot) = world.get::<ItemEquipmentSlot>(self.item) else {
            warn!("Item {:?} cannot be equipped", self.item);
            return;
        };

        match slot {
            ItemEquipmentSlot::Helmet => equip::<Helmet>(world, self.character, self.item),
            ItemEquipmentSlot::Chest => equip::<Chest>(world, self.character, self.item),
            ItemEquipmentSlot::Weapon => equip::<Weapon>(world, self.character, self.item),
        }
    }
}

fn equip<S: EquipmentSlot>(world: &mut World, character: Entity, item: Entity) {
    if let Some(previous_item) = world
        .get::<CharacterEquipment<S>>(character)
        .unwrap()
        .entity
    {
        UnequipItem {
            character,
            item: previous_item,
        }
        .apply(world);
    }

    world
        .get_mut::<CharacterEquipment<S>>(character)
        .unwrap()
        .entity = Some(item);
}

#[derive(Debug)]
pub struct UnequipItem {
    pub character: Entity,
    pub item: Entity,
}
impl Command for UnequipItem {
    fn apply(self, world: &mut World) {
        info!("{:?}", self);
        debug_assert!(world
            .get::<Parent>(self.item)
            .is_some_and(|p| p.get() == self.character));

        let slot = world.get::<ItemEquipmentSlot>(self.item).unwrap();

        match slot {
            ItemEquipmentSlot::Helmet => unequip::<Helmet>(world, self.character),
            ItemEquipmentSlot::Chest => unequip::<Chest>(world, self.character),
            ItemEquipmentSlot::Weapon => unequip::<Weapon>(world, self.character),
        }
    }
}

fn unequip<S: EquipmentSlot>(world: &mut World, character: Entity) {
    let mut equipment = world.get_mut::<CharacterEquipment<S>>(character).unwrap();
    equipment.entity = None;
}
