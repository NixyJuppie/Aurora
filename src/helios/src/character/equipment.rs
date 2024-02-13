use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct CharacterWeapon(Option<Entity>);

#[derive(Component, Default, Debug)]
pub struct CharacterArmor(Option<Entity>);
