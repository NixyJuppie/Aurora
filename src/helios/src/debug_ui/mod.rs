mod player;

use crate::schedule::InGameSet;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use player::draw_player_info;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, draw_player_info.in_set(InGameSet::DrawUi));
    }
}
