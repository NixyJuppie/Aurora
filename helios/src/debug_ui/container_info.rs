use crate::character::inventory::PickupItemCommand;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::debug_ui::{draw_grid, draw_row, FocusedEntity};
use crate::item::container::ItemContainerName;
use crate::item::ItemName;
use crate::player::Player;

pub fn draw_container_info(
    containers: Query<(Entity, Option<&Children>, &ItemContainerName)>,
    items: Query<(Entity, &ItemName), With<Parent>>,
    player: Query<Entity, With<Player>>,
    mut focus: ResMut<FocusedEntity>,
    mut egui_contexts: EguiContexts,
    mut commands: Commands,
) {
    let Some(container) = focus.0.and_then(|e| containers.get(e).ok()) else {
        return;
    };

    let (container, container_items, name) = container;

    egui::Window::new(&name.0)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .show(egui_contexts.ctx_mut(), |ui| {
            draw_grid("Base", ui, |ui| {
                draw_row(ui, "Id:", |ui| {
                    ui.label(&format!("{:?}", container));
                })
            });

            draw_grid("Items", ui, |ui| {
                if let Some(container_items) = container_items {
                    for (item, name) in container_items.iter().map(|i| items.get(*i).unwrap()) {
                        draw_row(ui, &name.0, |ui| {
                            if ui.button("Focus").clicked() {
                                focus.0 = Some(item);
                            }

                            if let Some(player) = player.iter().next() {
                                if ui.button("Loot (Player)").clicked() {
                                    commands.add(PickupItemCommand {
                                        item,
                                        character: player,
                                    });
                                }
                            }
                        });
                    }
                }
            });
        });
}
