use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
        app.configure_sets(
            Update,
            (InGameSet::UserInput, InGameSet::EntityUpdate)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(States, Hash, Default, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
    #[default]
    InGame,
}

#[derive(SystemSet, Hash, Clone, PartialEq, Eq, Debug)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    DrawUi,
}
