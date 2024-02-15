#![allow(clippy::type_complexity)]

pub use bevy;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_screen_diagnostics::*;
use character::player::PlayerPlugin;
use console_commands::ConsoleCommandsPlugin;
use debug_ui::DebugUiPlugin;
use input::InputPlugin;
use schedule::SchedulePlugin;

mod debug_ui;

pub mod character;
mod console_commands;
pub mod input;
pub mod item;
pub mod schedule;

pub struct HeliosPlugins;
impl PluginGroup for HeliosPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // external
            .add(ScreenDiagnosticsPlugin::default())
            .add(ScreenFrameDiagnosticsPlugin)
            .add(ScreenEntityDiagnosticsPlugin)
            // internal
            .add(ConsoleCommandsPlugin)
            .add(SchedulePlugin)
            .add(InputPlugin)
            .add(PlayerPlugin)
            .add(DebugUiPlugin)
    }
}
