use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::character::inventory::PickupItem;
use crate::character::player::Player;
use crate::debug_ui::{draw_grid, draw_row, FocusedEntity};
use crate::item::armor::ArmorProtection;
use crate::item::weapon::{WeaponDamage, WeaponRange};
use crate::item::{ItemEquipmentSlot, ItemName};

pub fn draw_world_item_info(
    focus: Res<FocusedEntity>,
    items: Query<
        (
            Entity,
            &ItemName,
            &ItemEquipmentSlot,
            Option<(&WeaponDamage, &WeaponRange)>,
            Option<&ArmorProtection>,
        ),
        Without<Parent>,
    >,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
) {
    let Some(item) = focus.0.and_then(|e| items.get(e).ok()) else {
        return;
    };

    let (item, name, slot, weapon, armor) = item;
    let player = player.single();

    egui::Window::new("Item")
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .show(egui_contexts.ctx_mut(), |ui| {
            draw_grid("Base", ui, |ui| {
                draw_row(ui, "Id:", |ui| {
                    ui.label(&format!("{:?}", item));
                });
                draw_row(ui, "Name:", |ui| {
                    ui.label(&name.0);
                });
                draw_row(ui, "Slot:", |ui| {
                    ui.label(format!("{:?}", slot));
                });
                draw_row(ui, "Actions:", |ui| {
                    if ui.button("Pickup").clicked() {
                        commands.add(PickupItem {
                            item,
                            character: player,
                        });
                    }
                });
            });

            if let Some((damage, range)) = weapon {
                draw_grid("Weapon", ui, |ui| {
                    draw_row(ui, "Damage:", |ui| {
                        ui.label(&format!("{} ({:?})", damage.damage, damage.damage_type));
                    });

                    draw_row(ui, "Range:", |ui| {
                        ui.label(&range.0.to_string());
                    });
                });
            }

            if let Some(protection) = armor {
                draw_grid("Armor", ui, |ui| {
                    draw_row(ui, "Physical protection:", |ui| {
                        ui.label(protection.physical.to_string());
                    });
                });
            }
        });
}
