use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub enum CharacterLoot {
    #[default]
    None,
    Fixed(Vec<Entity>),
}
