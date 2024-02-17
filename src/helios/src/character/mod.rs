use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smart_default::SmartDefault;

use attributes::{Agility, CharacterAttribute, Health, Strength};
use equipment::{CharacterEquipment, Chest, Helmet, Weapon};

pub mod attributes;
pub mod equipment;
pub mod player;

#[derive(Bundle, SmartDefault, Debug)]
pub struct CharacterBundle {
    #[default(CharacterName("Character".to_string()))]
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
}

#[derive(Component, Default, Debug)]
pub struct CharacterName(String);
