use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smart_default::SmartDefault;

use crate::character::attributes::{Agility, CharacterAttribute, Health, Strength};
use crate::character::equipment::{CharacterEquipment, Chest, Helmet, Weapon};
use crate::character::inventory::CharacterLoot;
use crate::character::{AttackCooldown, CharacterName};
use crate::HeliosCollision;

#[derive(Bundle, SmartDefault, Debug)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub loot: CharacterLoot,
    pub attack_cooldown: AttackCooldown,
    // attributes
    #[default(CharacterAttribute::new(100))]
    pub health: CharacterAttribute<Health>,
    pub strength: CharacterAttribute<Strength>,
    pub agility: CharacterAttribute<Agility>,
    // equipment
    pub helmet: CharacterEquipment<Helmet>,
    pub chest: CharacterEquipment<Chest>,
    pub weapon: CharacterEquipment<Weapon>,
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
    pub velocity: Velocity,
    #[default(KinematicCharacterController { filter_groups: Some(HeliosCollision::character_groups()), ..default() })]
    pub controller: KinematicCharacterController,
    #[default(HeliosCollision::character_groups())]
    pub collision_groups: CollisionGroups,
}
