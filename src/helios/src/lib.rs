use bevy::prelude::*;

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app
}
