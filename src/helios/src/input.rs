use crate::schedule::InGameSet;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InGameInput>();
        app.add_systems(Update, read_input.in_set(InGameSet::UserInput));
    }
}

#[derive(Resource, Default)]
pub struct InGameInput {
    pub movement: Vec2,
}

fn read_input(
    mut input: ResMut<InGameInput>,
    keyboard: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
) {
    input.movement = read_movement_input(&keyboard, &gamepads, &gamepad_axes);
}

fn read_movement_input(
    keyboard: &Res<Input<KeyCode>>,
    gamepads: &Res<Gamepads>,
    gamepad_axes: &Res<Axis<GamepadAxis>>,
) -> Vec2 {
    let mut movement = Vec2::ZERO;

    if keyboard.pressed(KeyCode::W) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        movement.x += 1.0;
    }

    const DEAD_ZONE: f32 = 0.1;
    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
            if x.abs() >= DEAD_ZONE {
                movement.x += x;
            }
        }
        if let Some(y) = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
            if y.abs() >= DEAD_ZONE {
                movement.y += y;
            }
        }
    }

    movement
}
