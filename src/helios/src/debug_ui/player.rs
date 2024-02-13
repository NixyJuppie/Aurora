use crate::character::attributes::{
    CharacterAgility, CharacterAttribute, CharacterHealth, CharacterStrength,
};
use crate::character::inventory::{CharacterArmor, CharacterWeapon, DropItem, EquipItem};
use crate::character::player::{FocusedWorldItem, Player};
use crate::character::{CharacterLookDirection, CharacterName};
use crate::item::{ItemName, WorldItem};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Color32, RichText, Ui, Widget};
use bevy_egui::{egui, EguiContexts};

pub fn draw_player_info(
    player: Query<
        (
            Entity,
            Option<&Children>,
            &CharacterName,
            &CharacterLookDirection,
            &CharacterHealth,
            &CharacterStrength,
            &CharacterAgility,
            &CharacterWeapon,
            &CharacterArmor,
        ),
        With<Player>,
    >,
    focused_item: Res<FocusedWorldItem>,
    mut items: Query<&ItemName, (With<Parent>, Without<WorldItem>)>,
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    let (player, children, name, look_direction, health, strength, agility, weapon, armor) =
        player.single();
    egui::Window::new(&name.0)
        .anchor(Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("Player").striped(true).show(ui, |ui| {
                ui.label("Health");
                draw_attribute(ui, Color32::DARK_RED, &health.0);
                ui.end_row();

                ui.label("Strength");
                draw_attribute(ui, Color32::DARK_GRAY, &strength.0);
                ui.end_row();

                ui.label("Agility");
                draw_attribute(ui, Color32::DARK_GREEN, &agility.0);
                ui.end_row();

                ui.label("Look direction");
                ui.label(look_direction.0.to_string());
                ui.end_row();

                ui.label("Weapon");
                ui.label(get_item_name(weapon.0, &items));
                ui.end_row();

                ui.label("Armor");
                ui.label(get_item_name(armor.0, &items));
                ui.end_row();

                ui.label("Focused item");
                ui.label(get_item_name(focused_item.0, &items));
            });

            if let Some(children) = children {
                egui::Grid::new("Items").striped(true).show(ui, |ui| {
                    ui.heading("Items");
                    ui.end_row();
                    for child in children.iter() {
                        if let Ok(name) = items.get_mut(*child) {
                            ui.label(&name.0);
                            if ui.button("Equip").clicked() {
                                commands.add(EquipItem {
                                    item: *child,
                                    character: player,
                                })
                            };
                            if ui.button("Drop").clicked() {
                                commands.add(DropItem {
                                    item: *child,
                                    character: player,
                                })
                            };
                            ui.end_row();
                        }
                    }
                });
            }
        });
}

fn get_item_name(
    item: Option<Entity>,
    items: &Query<&ItemName, (With<Parent>, Without<WorldItem>)>,
) -> String {
    match item {
        Some(item) => items
            .get(item)
            .map(|i| i.0.clone())
            .unwrap_or("Unknown".to_string()),
        None => "None".to_string(),
    }
}

fn draw_attribute(ui: &mut Ui, color: Color32, value: &CharacterAttribute) {
    egui::ProgressBar::new(value.current as f32 / value.max as f32)
        .fill(color)
        .desired_width(110.0)
        .text(RichText::new(format!(
            "{} / {} ({:.0}%)",
            value.current,
            value.max,
            (value.current as f32 / value.max as f32) * 100.0
        )))
        .ui(ui);
}
