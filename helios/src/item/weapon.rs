use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct WeaponRange(pub f32);

#[derive(Component, Default, Clone, Debug)]
pub struct WeaponDamage {
    pub damage: u32,
    pub damage_type: DamageType,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum DamageType {
    #[default]
    Physical,
}
