use bevy::prelude::*;
use damage::{ArmorProtection, WeaponDamage};
use smart_default::SmartDefault;

pub mod damage;

#[derive(Bundle, SmartDefault, Debug)]
pub struct ItemBundle {
    pub name: ItemName,
}

#[derive(Component, SmartDefault, Debug)]
pub struct ItemName(#[default("Item")] pub String);

#[derive(Bundle, SmartDefault, Debug)]
pub struct WeaponBundle {
    pub name: ItemName,
    pub damage: WeaponDamage,
    pub range: WeaponRange,
}

#[derive(Component, Default, Debug)]
pub struct WeaponRange(pub f32);

#[derive(Bundle, SmartDefault, Debug)]
pub struct ArmorBundle {
    pub name: ItemName,
    pub protection: ArmorProtection,
}
