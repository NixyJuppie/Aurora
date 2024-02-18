use crate::character::attack::AttackCommand;
use crate::character::equipment::{CharacterEquipment, Weapon};
use crate::character::player::Player;
use bevy::prelude::*;
use smart_default::SmartDefault;

pub mod attack;
pub mod attributes;
pub mod bundles;
pub mod equipment;
pub mod inventory;
pub mod player;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_attack_cooldown);
        app.add_systems(Update, temp_attack);
    }
}

#[derive(Component, SmartDefault, Debug)]
pub struct CharacterName(#[default("Character")] pub String);

#[derive(Component, SmartDefault, Debug)]
pub struct AttackCooldown(#[default(Timer::from_seconds(1.0, TimerMode::Once))] pub Timer);

fn update_attack_cooldown(mut query: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}

fn temp_attack(
    characters: Query<(Entity, &CharacterEquipment<Weapon>, &AttackCooldown), Without<Player>>,
    mut commands: Commands,
) {
    for (character, _, _) in characters
        .iter()
        .filter(|(_, w, c)| w.entity.is_some() && c.0.finished())
    {
        commands.add(AttackCommand {
            attacker: character,
        });
    }
}