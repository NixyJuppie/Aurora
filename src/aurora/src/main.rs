use helios::bevy::prelude::*;
use helios::camera::{GameCamera, GameCameraTarget};
use helios::create_app;
use helios::player::Player;

fn main() {
    create_app()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Camera2dBundle::default(), GameCamera));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        color: Color::GOLD,
                        ..default()
                    },
                    ..default()
                },
                GameCameraTarget { offset: Vec3::ZERO },
                Player,
            ));
        })
        .run();
}
