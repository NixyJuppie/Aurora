use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component, Default, Debug)]
pub struct CharacterEquipment<S> {
    slot_marker: PhantomData<S>,
    pub entity: Option<Entity>,
}

#[derive(Default, Debug)]
pub struct Chest;

#[derive(Default, Debug)]
pub struct Helmet;

#[derive(Default, Debug)]
pub struct Weapon;
