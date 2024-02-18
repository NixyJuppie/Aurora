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

pub trait EquipmentSlot: Send + Sync {}
impl EquipmentSlot for Chest {}
impl EquipmentSlot for Helmet {}
impl EquipmentSlot for Weapon {}

#[derive(Default, Debug)]
pub struct Chest;

#[derive(Default, Debug)]
pub struct Helmet;

#[derive(Default, Debug)]
pub struct Weapon;
