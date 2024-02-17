use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component, Default, Debug)]
pub struct CharacterEquipment<S: EquipmentSlot> {
    slot_marker: PhantomData<S>,
    pub entity: Option<Entity>,
}

pub trait EquipmentSlot {}
impl EquipmentSlot for Chest {}
impl EquipmentSlot for Helmet {}
impl EquipmentSlot for Weapon {}

#[derive(Default, Debug)]
pub struct Chest;

#[derive(Default, Debug)]
pub struct Helmet;

#[derive(Default, Debug)]
pub struct Weapon;
