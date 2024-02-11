pub use bevy;
use bevy::prelude::*;
use camera::CameraPlugin;

pub mod camera;

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(CameraPlugin);
    app
}
