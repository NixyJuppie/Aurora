use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(Bundle, SmartDefault, Debug)]
pub struct ItemBundle {
    pub name: ItemName,
}

#[derive(Component, SmartDefault, Debug)]
pub struct ItemName(#[default("Item")] pub String);
