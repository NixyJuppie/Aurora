use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug)]
pub struct CharacterAttribute {
    #[default(100)]
    pub current: u32,
    #[default(100)]
    pub max: u32,
}

#[derive(Component, Default, Debug)]
pub struct CharacterHealth(pub CharacterAttribute);

#[derive(Component, Default, Debug)]
pub struct CharacterStrength(pub CharacterAttribute);

#[derive(Component, Default, Debug)]
pub struct CharacterAgility(pub CharacterAttribute);
