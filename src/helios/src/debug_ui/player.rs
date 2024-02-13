use crate::character::attributes::{
    CharacterAgility, CharacterAttribute, CharacterHealth, CharacterStrength,
};
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
        ),
        With<Player>,
    >,
    focused_item: Res<FocusedWorldItem>,
    world_items: Query<&ItemName, With<WorldItem>>,
    mut items: Query<(&ItemName, &mut Visibility), (With<Parent>, Without<WorldItem>)>,
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    let (player, children, name, look_direction, health, strength, agility) = player.single();
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

                ui.label("Focused item");
                match focused_item.0.and_then(|e| world_items.get(e).ok()) {
                    None => ui.label("None"),
                    Some(item) => ui.label(&item.0),
                }
            });

            if let Some(children) = children {
                egui::Grid::new("Items").striped(true).show(ui, |ui| {
                    ui.heading("Items");
                    ui.end_row();
                    for child in children.iter() {
                        if let Ok((name, mut visibility)) = items.get_mut(*child) {
                            ui.label(&name.0);
                            if ui.button("Equip").clicked() {
                                // todo
                            };
                            if ui.button("Drop").clicked() {
                                commands.entity(player).remove_children(&[*child]);
                                commands.entity(*child).insert(WorldItem);
                                *visibility = Visibility::Inherited;
                            };
                            ui.end_row();
                        }
                    }
                });
            }
        });
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
