use crate::character::attack::{DealDamage, Kill};
use crate::character::inventory::{DropItem, PickupItem};
use crate::character::CharacterName;
use crate::item::{ItemName, WorldItem};
use bevy::app::{App, Plugin};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;

pub struct ConsoleCommandsPlugin;
impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ConsolePlugin>() {
            app.add_plugins(ConsolePlugin);
        }

        app.insert_resource(ConsoleConfiguration {
            left_pos: 0.0,
            top_pos: 0.0,
            width: 500.0,
            height: 200.0,
            ..default()
        });
        app.add_console_command::<DealDamageCmd, _>(deal_damage);
        app.add_console_command::<KillCharacterCmd, _>(kill_character);
        app.add_console_command::<PickupItemCmd, _>(pickup_item);
        app.add_console_command::<DropItemCmd, _>(drop_item);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "damage", about = "Deals damage to character with given id")]
struct DealDamageCmd {
    pub character: String,
    pub damage: u32,
}

fn deal_damage(
    mut command: ConsoleCommand<DealDamageCmd>,
    mut commands: Commands,
    query: Query<(Entity, &CharacterName)>,
) {
    let Some(Ok(DealDamageCmd {
        character: target,
        damage,
    })) = command.take()
    else {
        return;
    };

    match get_entity_with_name(&target, query) {
        Some(entity) => commands.add(DealDamage {
            target: entity,
            damage,
        }),
        None => reply!(command, "Character {} does not exist", target),
    };
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "kill", about = "Kills character with given id")]
struct KillCharacterCmd {
    pub character: String,
}

fn kill_character(
    mut command: ConsoleCommand<KillCharacterCmd>,
    mut commands: Commands,
    query: Query<(Entity, &CharacterName)>,
) {
    let Some(Ok(KillCharacterCmd { character: target })) = command.take() else {
        return;
    };

    match get_entity_with_name(&target, query) {
        Some(entity) => commands.add(Kill { character: entity }),
        None => reply!(command, "Character {} does not exist", target),
    };
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "pickup", about = "Pickups item from world")]
struct PickupItemCmd {
    pub character: String,
    pub item: String,
}

fn pickup_item(
    mut command: ConsoleCommand<PickupItemCmd>,
    mut commands: Commands,
    characters: Query<(Entity, &CharacterName)>,
    items: Query<(Entity, &ItemName), With<WorldItem>>,
) {
    let Some(Ok(PickupItemCmd { character, item })) = command.take() else {
        return;
    };

    match get_entity_with_name(&character, characters) {
        Some(character) => match get_entity_with_name(&item, items) {
            Some(item) => commands.add(PickupItem { item, character }),
            None => reply!(command, "Item {} does not exist", item),
        },
        None => reply!(command, "Character {} does not exist", character),
    };
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "drop", about = "Drops item from character inventory")]
struct DropItemCmd {
    pub character: String,
    pub item: String,
}

fn drop_item(
    mut command: ConsoleCommand<DropItemCmd>,
    mut commands: Commands,
    characters: Query<(Entity, &CharacterName)>,
    items: Query<(Entity, &ItemName)>,
) {
    let Some(Ok(DropItemCmd { character, item })) = command.take() else {
        return;
    };

    match get_entity_with_name(&character, characters) {
        Some(character) => match get_entity_with_name(&item, items) {
            Some(item) => commands.add(DropItem { item, character }),
            None => reply!(
                command,
                "Character does not have item {} in inventory",
                item
            ),
        },
        None => reply!(command, "Character {} does not exist", character),
    };
}

fn get_entity_with_name<N: Component + PartialEq<str>, F: ReadOnlyWorldQuery>(
    name: &str,
    query: Query<(Entity, &N), F>,
) -> Option<Entity> {
    query.iter().find(|(_, n)| (*n).eq(name)).map(|(e, _)| e)
}
