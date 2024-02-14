use crate::item::{ItemEquipSlot, WorldItem};
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct CharacterWeapon(pub Option<Entity>);

#[derive(Component, Default, Debug)]
pub struct CharacterArmor(pub Option<Entity>);

pub struct PickupItem {
    pub item: Entity,
    pub character: Entity,
}
impl Command for PickupItem {
    fn apply(self, world: &mut World) {
        let mut character = world.entity_mut(self.character);
        character.add_child(self.item);

        let mut item = world.entity_mut(self.item);
        item.remove::<WorldItem>();
        *item.get_mut::<Visibility>().unwrap() = Visibility::Hidden;
        item.get_mut::<Transform>().unwrap().translation = Vec3::ZERO;
    }
}

pub struct DropItem {
    pub item: Entity,
    pub character: Entity,
}
impl Command for DropItem {
    fn apply(self, world: &mut World) {
        let mut character = world.entity_mut(self.character);
        character.remove_children(&[self.item]);

        UnequipItem {
            item: self.item,
            character: self.character,
        }
        .apply(world);

        let mut item = world.entity_mut(self.item);
        item.insert(WorldItem);
        *item.get_mut::<Visibility>().unwrap() = Visibility::Inherited;
    }
}

pub struct EquipItem {
    pub item: Entity,
    pub character: Entity,
}
impl Command for EquipItem {
    fn apply(self, world: &mut World) {
        let item = world.entity(self.item);
        let Some(slot) = item.get::<ItemEquipSlot>().cloned() else {
            warn!("Cannot equip item because it does not have defined slot");
            return;
        };

        let mut character = world.entity_mut(self.character);
        match slot {
            ItemEquipSlot::Weapon => match character.get_mut::<CharacterWeapon>() {
                Some(mut weapon) => weapon.0 = Some(self.item),
                None => warn!(
                    "Cannot equip weapon because character does not have CharacterWeapon component"
                ),
            },
            ItemEquipSlot::Armor => match character.get_mut::<CharacterArmor>() {
                Some(mut armor) => armor.0 = Some(self.item),
                None => warn!(
                    "Cannot equip armor because character does not have CharacterArmor component"
                ),
            },
        }
    }
}

pub struct UnequipItem {
    pub item: Entity,
    pub character: Entity,
}
impl Command for UnequipItem {
    fn apply(self, world: &mut World) {
        let item = world.entity(self.item);
        let Some(slot) = item.get::<ItemEquipSlot>().cloned() else {
            warn!("Cannot unequip item because it does not have defined slot");
            return;
        };

        let mut character = world.entity_mut(self.character);
        match slot {
            ItemEquipSlot::Weapon => match character.get_mut::<CharacterWeapon>() {
                Some(mut weapon) => {
                    if weapon.0.is_some_and(|e| e == self.item) {
                        weapon.0 = None;
                    }
                },
                None => warn!(
                    "Cannot unequip weapon because character does not have CharacterWeapon component"
                ),
            },
            ItemEquipSlot::Armor => match character.get_mut::<CharacterArmor>() {
                Some(mut armor) => {
                    if armor.0.is_some_and(|e| e == self.item) {
                        armor.0 = None;
                    }
                },
                None => warn!(
                    "Cannot unequip armor because character does not have CharacterArmor component"
                ),
            },
        }
    }
}
