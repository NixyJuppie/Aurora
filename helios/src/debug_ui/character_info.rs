use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::camera::FollowTargetCommand;
use crate::character::attributes::{Agility, CharacterAttribute, Health, Strength};
use crate::character::equipment::{
    CharacterEquipment, Chest, EquipItemCommand, EquipmentSlot, Helmet, UnequipItemCommand, Weapon,
};
use crate::character::inventory::DropItemCommand;
use crate::character::CharacterName;
use crate::debug_ui::{draw_grid, draw_progress, draw_row, FocusedEntity};
use crate::item::ItemName;
use crate::player::ImpersonateCommand;

pub fn draw_character_info(
    characters: Query<(
        Entity,
        &CharacterName,
        &CharacterAttribute<Health>,
        &CharacterAttribute<Strength>,
        &CharacterAttribute<Agility>,
        &CharacterEquipment<Helmet>,
        &CharacterEquipment<Chest>,
        &CharacterEquipment<Weapon>,
    )>,
    items: Query<(Entity, &Parent, &ItemName)>,
    mut focus: ResMut<FocusedEntity>,
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
) {
    let Some(character) = focus.0.and_then(|e| characters.get(e).ok()) else {
        return;
    };

    let (character, name, health, strength, agility, helmet, chest, weapon) = character;

    egui::Window::new(&name.0)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .show(egui_contexts.ctx_mut(), |ui| {
            draw_grid("Base", ui, |ui| {
                draw_row(ui, "Id:", |ui| {
                    ui.label(&format!("{:?}", character));
                    if ui.button("Follow").clicked() {
                        commands.add(FollowTargetCommand(character));
                    }
                    if ui.button("Impersonate").clicked() {
                        commands.add(ImpersonateCommand(character));
                    }
                });
            });

            draw_grid("Attributes", ui, |ui| {
                draw_progress(
                    ui,
                    "Health:",
                    health.current as f32,
                    health.max as f32,
                    egui::Color32::RED,
                );
                draw_progress(
                    ui,
                    "Strength:",
                    strength.current as f32,
                    strength.max as f32,
                    egui::Color32::DARK_GRAY,
                );
                draw_progress(
                    ui,
                    "Agility:",
                    agility.current as f32,
                    agility.max as f32,
                    egui::Color32::DARK_GREEN,
                );
            });

            draw_grid("Equipment", ui, |ui| {
                draw_row(ui, "Helmet:", |ui| {
                    draw_equipment_item(&mut focus, &mut commands, ui, &items, character, helmet);
                });
                draw_row(ui, "Chest:", |ui| {
                    draw_equipment_item(&mut focus, &mut commands, ui, &items, character, chest);
                });
                draw_row(ui, "Weapon:", |ui| {
                    draw_equipment_item(&mut focus, &mut commands, ui, &items, character, weapon);
                });
            });

            draw_grid("Inventory", ui, |ui| {
                for (item, _, name) in items.iter().filter(|(_, p, _)| p.get() == character) {
                    draw_row(ui, &name.0, |ui| {
                        if ui.button("Focus").clicked() {
                            focus.0 = Some(item);
                        }
                        if ui.button("Equip").clicked() {
                            commands.add(EquipItemCommand { character, item });
                        }
                        if ui.button("Drop").clicked() {
                            commands.add(DropItemCommand { character, item });
                        }
                    });
                }
            });
        });
}

fn draw_equipment_item(
    focus: &mut ResMut<FocusedEntity>,
    commands: &mut Commands,
    ui: &mut egui::Ui,
    items: &Query<(Entity, &Parent, &ItemName)>,
    character: Entity,
    equipment: &CharacterEquipment<impl EquipmentSlot>,
) {
    ui.label(&get_item_name(equipment.entity, items));
    if let Some(item) = equipment.entity {
        if ui.button("Focus").clicked() {
            focus.0 = Some(item);
        }
        if ui.button("Unequip").clicked() {
            commands.add(UnequipItemCommand { character, item });
        }
    }
}

fn get_item_name(item: Option<Entity>, items: &Query<(Entity, &Parent, &ItemName)>) -> String {
    match item {
        Some(item) => match items.get(item) {
            Ok((_, _, name)) => name.0.clone(),
            Err(err) => format!("Unknown ({})", err),
        },
        None => "None".to_string(),
    }
}
