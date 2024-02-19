mod character_info;
mod item_info;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::RichText;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;

use crate::camera::GameCamera;
use crate::character::equipment::{CharacterEquipment, Weapon};
use crate::character::inventory::PICKUP_RADIUS;
use crate::character::CharacterName;
use crate::debug_ui::character_info::draw_character_info;
use crate::debug_ui::item_info::draw_item_info;
use crate::item::weapon::WeaponRange;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_resource::<FocusedEntity>();
        app.add_systems(Update, draw_character_weapon_range_gizmos);
        app.add_systems(Update, draw_pickup_range_gizmos);
        app.add_systems(Update, focus_entity);
        app.add_systems(Update, draw_character_info);
        app.add_systems(Update, draw_item_info);
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

fn draw_pickup_range_gizmos(
    characters: Query<&GlobalTransform, With<CharacterName>>,
    mut gizmos: Gizmos,
) {
    for character in characters.iter() {
        gizmos.sphere(
            character.translation(),
            Quat::default(),
            PICKUP_RADIUS,
            Color::DARK_GRAY,
        );
    }
}

#[derive(Resource, Default, Debug)]
struct FocusedEntity(Option<Entity>);

fn focus_entity(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    rapier: Res<RapierContext>,
    input: Res<ButtonInput<MouseButton>>,
    mut egui_contexts: EguiContexts,
    mut focus: ResMut<FocusedEntity>,
) {
    if egui_contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    if input.just_pressed(MouseButton::Right) {
        focus.0 = None;
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        let window = window.single();
        let (camera, camera_transform) = camera.single();
        if let Some((target, _)) = window
            .cursor_position()
            .and_then(|position| camera.viewport_to_world(camera_transform, position))
            .and_then(|ray| {
                rapier.cast_ray(
                    ray.origin,
                    ray.direction.into(),
                    25.0,
                    true,
                    QueryFilter::new(),
                )
            })
        {
            focus.0 = Some(target);
        }
    }
}

pub fn draw_grid(header: &str, ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    ui.heading(RichText::new(header).strong());
    egui::Grid::new(header).striped(true).show(ui, content);
}

pub fn draw_row(ui: &mut egui::Ui, label: &str, draw_content: impl FnOnce(&mut egui::Ui)) {
    ui.label(RichText::new(label).strong());
    ui.horizontal(|ui| draw_content(ui));
    ui.end_row();
}

pub fn draw_progress(ui: &mut egui::Ui, label: &str, current: f32, max: f32, color: egui::Color32) {
    ui.label(RichText::new(label).strong());
    ui.add(
        egui::ProgressBar::new(current / max)
            .text(format!("{} / {}", current, max))
            .fill(color),
    );
    ui.end_row();
}
