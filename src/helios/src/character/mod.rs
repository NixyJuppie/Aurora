use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smart_default::SmartDefault;

use crate::HeliosCollision;
use attributes::{Agility, CharacterAttribute, Health, Strength};
use equipment::{CharacterEquipment, Chest, Helmet, Weapon};

mod attack;
pub mod attributes;
pub mod equipment;
pub mod player;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_attack_cooldown);
    }
}

pub const CHARACTER_COLLISION_GROUP: u32 = 0b1;

#[derive(Bundle, SmartDefault, Debug)]
pub struct CharacterBundle {
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

#[derive(Component, SmartDefault, Debug)]
pub struct CharacterName(#[default("Character")] pub String);

#[derive(Component, SmartDefault, Debug)]
pub struct AttackCooldown(#[default(Timer::from_seconds(1.0, TimerMode::Once))] pub Timer);

fn update_attack_cooldown(mut query: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}
