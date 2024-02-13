#![allow(clippy::type_complexity)]

pub use bevy;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_screen_diagnostics::*;
use character::player::PlayerPlugin;
use debug_ui::DebugUiPlugin;
use input::InputPlugin;
use schedule::SchedulePlugin;

mod debug_ui;

pub mod character;
pub mod input;
pub mod item;
pub mod schedule;

pub struct HeliosPlugins;
impl PluginGroup for HeliosPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SchedulePlugin)
            .add(InputPlugin)
            .add(PlayerPlugin)
            .add(EguiPlugin)
            .add(DebugUiPlugin)
            .add(ScreenDiagnosticsPlugin::default())
            .add(ScreenFrameDiagnosticsPlugin)
            .add(ScreenEntityDiagnosticsPlugin)
    }
}
