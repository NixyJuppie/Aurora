use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameplayInput>();
        app.add_systems(Update, update_gameplay_input);
    }
}

#[derive(Resource, Default, Debug)]
pub struct GameplayInput {
    pub movement: Option<Vec2>,
    pub look: Option<Vec2>,
    pub attack: bool,
}

fn update_gameplay_input(
    mut input: ResMut<GameplayInput>,
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    keyboard: Res<Input<KeyCode>>,
    mouse_motion: EventReader<MouseMotion>,
) {
    input.movement = merge_inputs(
        read_gamepad_vec2(
            &gamepads,
            &gamepad_axes,
            GamepadAxisType::LeftStickX,
            GamepadAxisType::LeftStickY,
        ),
        read_keyboard_vec2(&keyboard, KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D),
    );

    input.look = merge_inputs(
        read_gamepad_vec2(
            &gamepads,
            &gamepad_axes,
            GamepadAxisType::RightStickX,
            GamepadAxisType::RightStickY,
        ),
        read_mouse_motion(mouse_motion).map(|v| Vec2::new(v.x, -v.y)),
    );

    input.attack = gamepads
        .iter()
        .map(|g| gamepad_buttons.pressed(GamepadButton::new(g, GamepadButtonType::RightTrigger)))
        .any(|v| v)
        || keyboard.pressed(KeyCode::E);
}

fn merge_inputs(first: Option<Vec2>, second: Option<Vec2>) -> Option<Vec2> {
    if first.is_some() {
        first
    } else {
        second
    }
}

fn read_gamepad_vec2(
    gamepads: &Res<Gamepads>,
    gamepad_axis: &Res<Axis<GamepadAxis>>,
    horizontal: GamepadAxisType,
    vertical: GamepadAxisType,
) -> Option<Vec2> {
    const DEAD_ZONE: f32 = 0.1;

    for gamepad in gamepads.iter() {
        if let (Some(x), Some(y)) = (
            gamepad_axis.get(GamepadAxis::new(gamepad, horizontal)),
            gamepad_axis.get(GamepadAxis::new(gamepad, vertical)),
        ) {
            let value = Vec2::new(x, y);
            if value.length() >= DEAD_ZONE {
                return Some(value);
            }
        }
    }

    None
}

fn read_keyboard_vec2(
    keyboard: &Res<Input<KeyCode>>,
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
) -> Option<Vec2> {
    let mut value = Vec2::ZERO;

    if keyboard.pressed(up) {
        value.y += 1.0;
    }
    if keyboard.pressed(down) {
        value.y -= 1.0;
    }
    if keyboard.pressed(left) {
        value.x -= 1.0;
    }
    if keyboard.pressed(right) {
        value.x += 1.0;
    }

    if value == Vec2::ZERO {
        None
    } else {
        Some(value)
    }
}

fn read_mouse_motion(mut motion: EventReader<MouseMotion>) -> Option<Vec2> {
    const MOUSE_SPEED: f32 = 0.2;
    let value: Vec2 = motion.read().map(|e| e.delta).sum();

    if value != Vec2::ZERO {
        Some(value * MOUSE_SPEED)
    } else {
        None
    }
}
