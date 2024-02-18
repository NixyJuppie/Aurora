use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;

use crate::camera::GameCamera;
use crate::character::attributes::{Agility, CharacterAttribute, Health, Strength};
use crate::character::equipment::{CharacterEquipment, Chest, Helmet, Weapon};
use crate::character::CharacterName;
use crate::item::{ItemName, WeaponRange};

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, draw_focused_entity_info);
        app.add_systems(Update, draw_character_weapon_range_gizmos);
    }
}

type CharacterComponents<'a> = (
    Entity,
    &'a CharacterName,
    &'a CharacterAttribute<Health>,
    &'a CharacterAttribute<Strength>,
    &'a CharacterAttribute<Agility>,
    &'a CharacterEquipment<Helmet>,
    &'a CharacterEquipment<Chest>,
    &'a CharacterEquipment<Weapon>,
);

type ItemComponents<'a> = (Entity, &'a Parent, &'a ItemName);

fn draw_focused_entity_info(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    characters: Query<CharacterComponents>,
    items: Query<ItemComponents>,
    rapier: Res<RapierContext>,
    mut egui_contexts: EguiContexts,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    if let Some((target, _)) = window
        .cursor_position()
        .and_then(|position| camera.viewport_to_world(camera_transform, position))
        .and_then(|ray| rapier.cast_ray(ray.origin, ray.direction, 25.0, true, QueryFilter::new()))
    {
        if let Ok(character) = characters.get(target) {
            egui::Window::new("Character")
                .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
                .show(egui_contexts.ctx_mut(), |ui| {
                    draw_character_info(ui, character, &items)
                });
        }
    }
}

fn draw_character_weapon_range_gizmos(
    characters: Query<(&Transform, &CharacterEquipment<Weapon>), With<CharacterName>>,
    weapons: Query<&WeaponRange, With<Parent>>,
    mut gizmos: Gizmos,
) {
    for (character, weapon) in characters.iter() {
        if let Some(range) = weapon.entity.and_then(|w| weapons.get(w).ok()) {
            let radius = range.0 / 2.0;
            gizmos.sphere(
                character.translation + (character.forward() * radius),
                character.rotation,
                radius,
                Color::RED,
            );
        }
    }
}

fn draw_character_info(
    ui: &mut egui::Ui,
    character: CharacterComponents,
    items: &Query<ItemComponents>,
) {
    let (character, name, health, strength, agility, helmet, chest, weapon) = character;

    grid("Base", ui, |ui| {
        draw_string(ui, "Name:", &name.0);
    });

    grid("Attributes", ui, |ui| {
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

    grid("Equipment", ui, |ui| {
        draw_string(ui, "Helmet", &get_item_name(helmet.entity, items));
        draw_string(ui, "Chest", &get_item_name(chest.entity, items));
        draw_string(ui, "Weapon", &get_item_name(weapon.entity, items));
    });

    grid("Inventory", ui, |ui| {
        for (item, _, name) in items.iter().filter(|(_, p, _)| p.get() == character) {
            draw_string(ui, &format!("{:?}", item), &name.0);
        }
    });
}

fn get_item_name(item: Option<Entity>, items: &Query<ItemComponents>) -> String {
    match item {
        Some(item) => match items.get(item) {
            Ok((_, _, name)) => name.0.clone(),
            Err(err) => format!("Unknown ({})", err),
        },
        None => "None".to_string(),
    }
}

fn grid(header: &str, ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    ui.heading(header);
    egui::Grid::new(header).striped(true).show(ui, content);
}

fn draw_string(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(label);
    ui.label(value);
    ui.end_row();
}

fn draw_progress(ui: &mut egui::Ui, label: &str, current: f32, max: f32, color: egui::Color32) {
    ui.label(label);
    ui.add(
        egui::ProgressBar::new(current / max)
            .text(format!("{} / {}", current, max))
            .fill(color),
    );
    ui.end_row();
}
