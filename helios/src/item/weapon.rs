use bevy::prelude::*;
use bevy::utils::smallvec::SmallVec;

#[derive(Component, Default, Debug)]
pub struct WeaponRange(pub f32);

#[derive(Component, Default, Clone, Debug)]
pub struct WeaponDamage(pub SmallVec<[Damage; 2]>);

#[derive(Default, Debug, Copy, Clone)]
pub struct Damage {
    pub damage: u32,
    pub damage_type: DamageType,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum DamageType {
    #[default]
    Physical,
    Fire,
}
