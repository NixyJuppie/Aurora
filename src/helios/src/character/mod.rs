use crate::character::inventory::{CharacterArmor, CharacterWeapon};
use attributes::{CharacterAgility, CharacterHealth, CharacterStrength};
use bevy::prelude::*;

pub mod attributes;
pub mod inventory;
pub mod player;

#[derive(Bundle, Default, Debug)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub look_direction: CharacterLookDirection,

    pub health: CharacterHealth,
    pub strength: CharacterStrength,
    pub agility: CharacterAgility,

    pub weapon: CharacterWeapon,
    pub armor: CharacterArmor,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Component, Default, Debug)]
pub struct CharacterName(pub String);

#[derive(Component, Default, Debug)]
pub struct CharacterLookDirection(pub Vec2);
