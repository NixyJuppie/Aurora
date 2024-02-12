use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub health: CharacterHealth,
    pub strength: CharacterStrength,
    pub agility: CharacterAgility,
}

#[derive(Component, Default)]
pub struct CharacterName(pub String);

#[derive(Component, Default)]
pub struct CharacterHealth(pub CharacterAttribute);

#[derive(Component, Default)]
pub struct CharacterStrength(pub CharacterAttribute);

#[derive(Component, Default)]
pub struct CharacterAgility(pub CharacterAttribute);

#[derive(SmartDefault)]
pub struct CharacterAttribute {
    #[default(100)]
    pub current: u32,
    #[default(100)]
    pub max: u32,
}
