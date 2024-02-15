use crate::schedule::AutoDespawn;
use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct SpawnDamageIndicator {
    pub target: Entity,
    pub damage: u32,
}

impl Command for SpawnDamageIndicator {
    fn apply(self, world: &mut World) {
        let translation = world
            .get::<GlobalTransform>(self.target)
            .unwrap()
            .translation();

        world.spawn((
            Text2dBundle {
                transform: Transform::from_xyz(translation.x, translation.y, 10.0),
                text: Text::from_section(
                    self.damage.to_string(),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            },
            AutoDespawn(Timer::from_seconds(0.75, TimerMode::Once)),
        ));
    }
}
