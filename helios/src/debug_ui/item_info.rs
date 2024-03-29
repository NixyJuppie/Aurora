use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::character::equipment::EquipItemCommand;
use crate::character::inventory::{DropItemCommand, PickupItemCommand};
use crate::character::CharacterName;
use crate::debug_ui::{draw_grid, draw_row, FocusedEntity};
use crate::item::armor::ArmorProtection;
use crate::item::container::ItemContainerName;
use crate::item::weapon::{WeaponDamage, WeaponRange};
use crate::item::{ItemEquipmentSlot, ItemName};
use crate::player::Player;

pub fn draw_item_info(
    items: Query<(
        Entity,
        &ItemName,
        &ItemEquipmentSlot,
        Option<&Parent>,
        Option<(&WeaponDamage, &WeaponRange)>,
        Option<&ArmorProtection>,
    )>,
    characters: Query<(Entity, &CharacterName)>,
    containers: Query<(Entity, &ItemContainerName)>,
    player: Query<Entity, With<Player>>,
    mut focus: ResMut<FocusedEntity>,
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
) {
    let Some(item) = focus.0.and_then(|e| items.get(e).ok()) else {
        return;
    };

    let (item, name, slot, parent, weapon, armor) = item;

    egui::Window::new(&name.0)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .show(egui_contexts.ctx_mut(), |ui| {
            draw_grid("Base", ui, |ui| {
                draw_row(ui, "Id:", |ui| {
                    ui.label(&format!("{:?}", item));
                });
                if let Some(owner) = parent.map(|p| p.get()) {
                    draw_row(ui, "Owner:", |ui| {
                        if let Ok((character, name)) = characters.get(owner) {
                            ui.label(format!("{} ({:?})", name.0, character));
                            if ui.button("Focus").clicked() {
                                focus.0 = Some(character);
                            }
                        } else if let Ok((container, name)) = containers.get(owner) {
                            ui.label(format!("{} ({:?})", name.0, container));
                            if ui.button("Focus").clicked() {
                                focus.0 = Some(container);
                            }
                        } else {
                            ui.label(format!("Unknown ({:?})", owner));
                        }
                    });
                }
                draw_row(ui, "Slot:", |ui| {
                    ui.label(format!("{:?}", slot));
                });
                draw_row(ui, "Actions:", |ui| {
                    if let Some((owner, _)) = parent.and_then(|x| characters.get(x.get()).ok()) {
                        if ui.button("Equip").clicked() {
                            commands.add(EquipItemCommand {
                                item,
                                character: owner,
                            });
                        }

                        if ui.button("Drop").clicked() {
                            commands.add(DropItemCommand {
                                item,
                                character: owner,
                            });
                        }
                    } else if let Some(player) = player.iter().next() {
                        if ui.button("Pickup (Player)").clicked() {
                            commands.add(PickupItemCommand {
                                item,
                                character: player,
                            });
                        }
                    }
                });
            });

            if let Some((damage, range)) = weapon {
                draw_grid("Weapon", ui, |ui| {
                    draw_row(ui, "Damage:", |ui| {
                        ui.label(
                            damage
                                .0
                                .iter()
                                .map(|d| format!("{} ({:?})", d.damage, d.damage_type))
                                .collect::<Vec<String>>()
                                .join(", "),
                        );
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
