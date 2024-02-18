use crate::character::attack::AttackCommand;
use crate::character::attributes::{Agility, CharacterAttribute, Health, Strength};
use crate::character::equipment::{CharacterEquipment, Chest, Helmet, Weapon};
use crate::character::{AttackCooldown, CharacterName};
use crate::input::GameplayInput;
use crate::HeliosCollision;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smart_default::SmartDefault;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
        app.add_systems(Update, rotate_player);
        app.add_systems(Update, attack);
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Bundle, SmartDefault, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    #[default(CharacterName("Player".to_string()))]
    pub name: CharacterName,
    // attributes
    #[default(CharacterAttribute::new(100))]
    pub health: CharacterAttribute<Health>,
    pub strength: CharacterAttribute<Strength>,
    pub agility: CharacterAttribute<Agility>,
    // equipment
    pub helmet: CharacterEquipment<Helmet>,
    pub chest: CharacterEquipment<Chest>,
    pub weapon: CharacterEquipment<Weapon>,
    pub attack_cooldown: AttackCooldown,
    // core
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    // mesh
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    // physics
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    #[default(HeliosCollision::character_groups())]
    pub collision_groups: CollisionGroups,
}

fn move_player(
    mut players: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Some(movement) = input.movement else {
        return;
    };

    const METERS_PER_SECOND: f32 = 10.0;
    for mut player in players.iter_mut() {
        let movement =
            Vec3::new(movement.x, 0.0, -movement.y) * METERS_PER_SECOND * time.delta_seconds();
        player.translation = Some(movement);
    }
}

fn rotate_player(
    mut players: Query<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Some(rotation) = input.rotate else {
        return;
    };

    const DEGREES_PER_SECOND: f32 = 180.0;
    for mut player in players.iter_mut() {
        let rotation = -rotation * DEGREES_PER_SECOND * time.delta_seconds();
        player.rotate_y(rotation.to_radians());
    }
}

fn attack(
    mut commands: Commands,
    players: Query<(&AttackCooldown, Entity), With<Player>>,
    input: Res<GameplayInput>,
) {
    if !input.attack {
        return;
    }

    for (_, player) in players.iter().filter(|(c, _)| c.0.finished()) {
        commands.add(AttackCommand { attacker: player })
    }
}
