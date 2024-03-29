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
    pub rotate: Option<f32>,
    pub attack: bool,
    pub pickup: bool,
    pub jump: bool,
}

fn update_gameplay_input(
    mut input: ResMut<GameplayInput>,
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    input.movement = merge_inputs(
        read_gamepad_axes(
            &gamepads,
            &gamepad_axes,
            GamepadAxisType::LeftStickX,
            GamepadAxisType::LeftStickY,
        ),
        read_keyboard_axes(
            &keyboard,
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::KeyS,
            KeyCode::KeyW,
        ),
    );

    input.rotate = merge_inputs(
        read_gamepad_axis(&gamepads, &gamepad_axes, GamepadAxisType::RightStickX),
        read_keyboard_axis(&keyboard, KeyCode::KeyQ, KeyCode::KeyE),
    );

    input.attack = gamepads
        .iter()
        .map(|g| gamepad_buttons.pressed(GamepadButton::new(g, GamepadButtonType::RightTrigger)))
        .any(|v| v)
        || keyboard.pressed(KeyCode::ControlLeft);

    input.pickup = gamepads
        .iter()
        .map(|g| gamepad_buttons.pressed(GamepadButton::new(g, GamepadButtonType::South)))
        .any(|v| v)
        || keyboard.pressed(KeyCode::KeyF);

    input.jump = gamepads
        .iter()
        .map(|g| gamepad_buttons.pressed(GamepadButton::new(g, GamepadButtonType::East)))
        .any(|v| v)
        || keyboard.pressed(KeyCode::Space);
}

fn merge_inputs<T>(first: Option<T>, second: Option<T>) -> Option<T> {
    if first.is_some() {
        first
    } else {
        second
    }
}

fn read_gamepad_axis(
    gamepads: &Res<Gamepads>,
    gamepad_axis: &Res<Axis<GamepadAxis>>,
    axis: GamepadAxisType,
) -> Option<f32> {
    const DEAD_ZONE: f32 = 0.1;

    for gamepad in gamepads.iter() {
        if let Some(value) = gamepad_axis.get(GamepadAxis::new(gamepad, axis)) {
            if value.abs() >= DEAD_ZONE {
                return Some(value);
            }
        }
    }

    None
}

fn read_gamepad_axes(
    gamepads: &Res<Gamepads>,
    gamepad_axis: &Res<Axis<GamepadAxis>>,
    x_axis: GamepadAxisType,
    y_axis: GamepadAxisType,
) -> Option<Vec2> {
    let x = read_gamepad_axis(gamepads, gamepad_axis, x_axis);
    let y = read_gamepad_axis(gamepads, gamepad_axis, y_axis);

    if x.is_none() && y.is_none() {
        None
    } else {
        Some(Vec2::new(x.unwrap_or(0.0), y.unwrap_or(0.0)))
    }
}

fn read_keyboard_axis(
    keyboard: &Res<ButtonInput<KeyCode>>,
    negative: KeyCode,
    positive: KeyCode,
) -> Option<f32> {
    let negative = if keyboard.pressed(negative) { 1.0 } else { 0.0 };
    let positive = if keyboard.pressed(positive) { 1.0 } else { 0.0 };
    let value = positive - negative;

    if value == 0.0 {
        None
    } else {
        Some(value)
    }
}

fn read_keyboard_axes(
    keyboard: &Res<ButtonInput<KeyCode>>,
    negative_x: KeyCode,
    positive_x: KeyCode,
    negative_y: KeyCode,
    positive_y: KeyCode,
) -> Option<Vec2> {
    let x = read_keyboard_axis(keyboard, negative_x, positive_x);
    let y = read_keyboard_axis(keyboard, negative_y, positive_y);

    if x.is_none() && y.is_none() {
        None
    } else {
        Some(Vec2::new(x.unwrap_or(0.0), y.unwrap_or(0.0)))
    }
}
