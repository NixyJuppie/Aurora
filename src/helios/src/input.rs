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
    pub action: bool,
}

fn read_input(
    mut input: ResMut<InGameInput>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
) {
    let movement = read_keyboard_vec2(KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D, &keys)
        + read_gamepad_vec2(
            GamepadAxisType::LeftStickX,
            GamepadAxisType::LeftStickY,
            &gamepads,
            &gamepad_axes,
        );

    input.movement = if movement.length() > 1.0 {
        movement.normalize()
    } else {
        movement
    };

    input.action = keys.pressed(KeyCode::E)
        || gamepads
            .iter()
            .map(|g| buttons.pressed(GamepadButton::new(g, GamepadButtonType::South)))
            .any(|v| v)
}

fn read_gamepad_vec2(
    axis_x: GamepadAxisType,
    axis_y: GamepadAxisType,
    gamepads: &Res<Gamepads>,
    gamepad_axes: &Res<Axis<GamepadAxis>>,
) -> Vec2 {
    const DEAD_ZONE: f32 = 0.1;
    let mut value = Vec2::ZERO;

    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad_axes.get(GamepadAxis::new(gamepad, axis_x)) {
            if x.abs() >= DEAD_ZONE {
                value.x += x;
            }
        }
        if let Some(y) = gamepad_axes.get(GamepadAxis::new(gamepad, axis_y)) {
            if y.abs() >= DEAD_ZONE {
                value.y += y;
            }
        }
    }

    value
}

fn read_keyboard_vec2(
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    keys: &Res<Input<KeyCode>>,
) -> Vec2 {
    let mut value = Vec2::ZERO;

    if keys.pressed(up) {
        value.y += 1.0;
    }
    if keys.pressed(down) {
        value.y -= 1.0;
    }
    if keys.pressed(left) {
        value.x -= 1.0;
    }
    if keys.pressed(right) {
        value.x += 1.0;
    }

    value
}
