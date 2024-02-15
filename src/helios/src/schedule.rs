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
        app.add_systems(
            Update,
            auto_despawn_entities.in_set(InGameSet::EntityUpdate),
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

#[derive(Component)]
pub struct AutoDespawn(pub Timer);

fn auto_despawn_entities(
    mut query: Query<(Entity, &mut AutoDespawn)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (indicator, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(indicator).despawn();
        }
    }
}
