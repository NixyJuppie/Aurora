use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(Bundle, Default, Debug)]
pub struct ItemBundle {
    pub name: ItemName,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Component)]
pub struct WorldItem;

#[derive(Component, Default, Debug)]
pub struct ItemName(pub String);

impl PartialEq<str> for ItemName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Component, Debug, Clone)]
pub enum ItemEquipSlot {
    Weapon,
    Armor,
}

#[derive(Bundle, SmartDefault, Debug)]
pub struct WeaponBundle {
    pub name: ItemName,
    #[default(ItemEquipSlot::Weapon)]
    pub slot: ItemEquipSlot,
    pub damage: WeaponDamage,
    pub range: WeaponRange,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Component, Default, Debug)]
pub struct WeaponDamage(pub u32);

#[derive(Component, Default, Debug)]
pub struct WeaponRange(pub f32);

#[derive(Bundle, SmartDefault, Debug)]
pub struct ArmorBundle {
    pub name: ItemName,
    #[default(ItemEquipSlot::Armor)]
    pub slot: ItemEquipSlot,
    pub defense: ArmorDefense,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Component, Default, Debug)]
pub struct ArmorDefense(pub u32);
