use crate::character::{
    CharacterAgility, CharacterAttribute, CharacterHealth, CharacterName, CharacterStrength,
};
use crate::player::Player;
use crate::schedule::InGameSet;
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Color32, RichText, Ui, Widget};
use bevy_egui::{egui, EguiContexts};

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_player_info.in_set(InGameSet::DrawUi));
    }
}

fn draw_player_info(
    player: Query<
        (
            &CharacterName,
            &CharacterHealth,
            &CharacterStrength,
            &CharacterAgility,
        ),
        With<Player>,
    >,
    mut contexts: EguiContexts,
) {
    let (name, health, strength, agility) = player.single();
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
            });
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
