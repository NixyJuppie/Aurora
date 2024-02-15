use attributes::{CharacterAgility, CharacterHealth, CharacterStrength};
use bevy::prelude::*;
use inventory::{CharacterArmor, CharacterWeapon};
use loot::CharacterLoot;

mod attack;
pub mod attributes;
pub mod damage;
pub mod inventory;
pub mod loot;
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
    pub loot: CharacterLoot,

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
