use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smart_default::SmartDefault;

use crate::item::armor::ArmorProtection;
use crate::item::weapon::{WeaponDamage, WeaponRange};
use crate::item::{ItemEquipmentSlot, ItemName};
use crate::HeliosCollision;

#[derive(Bundle, SmartDefault, Debug)]
pub struct ItemBundle {
    pub name: ItemName,
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
    #[default(HeliosCollision::item_groups())]
    pub collision_groups: CollisionGroups,
}

#[derive(Bundle, SmartDefault, Debug)]
pub struct WeaponBundle {
    #[default(ItemName("Weapon".to_string()))]
    pub name: ItemName,
    #[default(ItemEquipmentSlot::Weapon)]
    pub slot: ItemEquipmentSlot,
    pub damage: WeaponDamage,
    pub range: WeaponRange,
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
    #[default(HeliosCollision::item_groups())]
    pub collision_groups: CollisionGroups,
}

#[derive(Bundle, SmartDefault, Debug)]
pub struct ArmorBundle {
    #[default(ItemName("Armor".to_string()))]
    pub name: ItemName,
    #[default(ItemEquipmentSlot::Chest)]
    pub slot: ItemEquipmentSlot,
    pub protection: ArmorProtection,
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
    #[default(HeliosCollision::item_groups())]
    pub collision_groups: CollisionGroups,
}
