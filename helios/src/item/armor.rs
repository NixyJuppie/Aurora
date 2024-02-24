use crate::item::weapon::DamageType;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct ArmorProtection {
    pub physical: u32,
    pub fire: u32,
}

impl ArmorProtection {
    pub fn get(&self, damage_type: DamageType) -> u32 {
        match damage_type {
            DamageType::Physical => self.physical,
            DamageType::Fire => self.fire,
        }
    }
}
