use bevy::prelude::*;
use smart_default::SmartDefault;
use std::marker::PhantomData;

#[derive(Component, SmartDefault, Debug)]
pub struct CharacterAttribute<A> {
    attribute_marker: PhantomData<A>,
    #[default(10)]
    pub current: u32,
    #[default(10)]
    pub max: u32,
}

impl<A> CharacterAttribute<A> {
    pub fn new(value: u32) -> Self {
        Self {
            attribute_marker: PhantomData,
            current: value,
            max: value,
        }
    }
}

#[derive(Default, Debug)]
pub struct Health;

#[derive(Default, Debug)]
pub struct Strength;

#[derive(Default, Debug)]
pub struct Agility;
