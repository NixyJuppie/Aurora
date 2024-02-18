use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub struct WeaponDamage {
    pub damage: u32,
    pub damage_type: DamageType,
}

#[derive(Component, Default, Debug)]
pub struct ArmorProtection {
    pub physical: u32,
}

impl ArmorProtection {
    pub fn get(&self, damage_type: DamageType) -> u32 {
        match damage_type {
            DamageType::Physical => self.physical,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum DamageType {
    #[default]
    Physical,
}
