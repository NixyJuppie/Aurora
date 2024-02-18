use bevy::prelude::*;
use smart_default::SmartDefault;

pub mod armor;
pub mod bundles;
pub mod weapon;

#[derive(Component, SmartDefault, Debug)]
pub struct ItemName(#[default("Item")] pub String);

#[derive(Component, Copy, Clone, Debug)]
pub enum ItemEquipmentSlot {
    Helmet,
    Chest,
    Weapon,
}
