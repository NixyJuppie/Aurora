pub use bevy;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_screen_diagnostics::*;

use camera::CameraPlugin;
use debug_ui::DebugUiPlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;

pub mod camera;
pub mod character;
pub mod debug_ui;
pub mod input;
pub mod player;
pub mod schedule;

pub struct HeliosPlugins;
impl PluginGroup for HeliosPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SchedulePlugin)
            .add(InputPlugin)
            .add(CameraPlugin)
            .add(PlayerPlugin)
            .add(EguiPlugin)
            .add(DebugUiPlugin)
            .add(ScreenDiagnosticsPlugin::default())
            .add(ScreenFrameDiagnosticsPlugin)
            .add(ScreenEntityDiagnosticsPlugin)
    }
}
