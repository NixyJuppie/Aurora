use crate::character::CharacterLookDirection;
use crate::input::InGameInput;
use crate::item::WorldItem;
use crate::schedule::InGameSet;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FocusedWorldItem>();
        app.add_systems(
            Update,
            (move_player, focus_world_item, pick_up_focused_item).in_set(InGameSet::EntityUpdate),
        );
    }
}

#[derive(Component, Debug)]
pub struct Player;

fn move_player(
    mut player: Query<(&mut Transform, &mut CharacterLookDirection), With<Player>>,
    input: Res<InGameInput>,
) {
    const SPEED: f32 = 5.0;

    if input.movement != Vec2::ZERO {
        let (mut transform, mut look_direction) = player.single_mut();
        transform.translation += Vec3::new(input.movement.x, input.movement.y, 0.0) * SPEED;
        look_direction.0 = input.movement.normalize_or_zero();
    }
}

#[derive(Resource, Default)]
pub struct FocusedWorldItem(pub Option<Entity>);

fn focus_world_item(
    mut focused_item: ResMut<FocusedWorldItem>,
    player: Query<(&GlobalTransform, &CharacterLookDirection), With<Player>>,
    items: Query<(&GlobalTransform, Entity), With<WorldItem>>,
) {
    const FOCUS_DISTANCE: f32 = 250.0;
    const FOCUS_ANGLE: f32 = 90.0;

    let (player_transform, look_direction) = player.single();
    for (item_transform, item_entity) in items.iter() {
        let direction = item_transform.translation() - player_transform.translation();

        if direction.length() <= FOCUS_DISTANCE {
            let angle = direction.angle_between(Vec3::new(
                look_direction.0.x,
                look_direction.0.y,
                direction.z,
            ));

            if angle.to_degrees().abs() <= FOCUS_ANGLE {
                focused_item.0 = Some(item_entity);
                return;
            }
        }
    }

    focused_item.0 = None;
}

fn pick_up_focused_item(
    mut commands: Commands,
    mut focused_item: ResMut<FocusedWorldItem>,
    player: Query<Entity, With<Player>>,
    mut items: Query<(&mut Transform, &mut Visibility, Entity), With<WorldItem>>,
    input: Res<InGameInput>,
) {
    if !input.action {
        return;
    }

    let Some((mut transform, mut visibility, item)) =
        focused_item.0.and_then(|e| items.get_mut(e).ok())
    else {
        return;
    };
    let player = player.single();

    commands.entity(item).remove::<WorldItem>();
    commands.entity(player).add_child(item);
    *visibility = Visibility::Hidden;
    transform.translation = Vec3::ZERO;

    focused_item.0 = None;
}
