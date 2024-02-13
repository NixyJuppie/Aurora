mod player;

use crate::schedule::InGameSet;
use bevy::prelude::*;
use player::draw_player_info;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_player_info.in_set(InGameSet::DrawUi));
    }
}
