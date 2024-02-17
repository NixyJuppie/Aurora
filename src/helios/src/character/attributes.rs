use bevy::prelude::*;
use smart_default::SmartDefault;
use std::marker::PhantomData;

#[derive(Component, SmartDefault, Debug)]
pub struct CharacterAttribute<A: Attribute> {
    attribute_marker: PhantomData<A>,
    #[default(10)]
    pub current: u32,
    #[default(10)]
    pub max: u32,
}

impl<A: Attribute> CharacterAttribute<A> {
    pub fn new(value: u32) -> Self {
        Self {
            attribute_marker: PhantomData,
            current: value,
            max: value,
        }
    }
}

pub trait Attribute {}
impl Attribute for Health {}
impl Attribute for Strength {}
impl Attribute for Agility {}

#[derive(Default, Debug)]
pub struct Health;

#[derive(Default, Debug)]
pub struct Strength;

#[derive(Default, Debug)]
pub struct Agility;
