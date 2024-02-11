pub use bevy;
use bevy::prelude::*;
use bevy_screen_diagnostics::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

pub mod camera;
pub mod player;

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(CameraPlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
        ScreenEntityDiagnosticsPlugin,
    ));
    app
}
