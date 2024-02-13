use crate::item::{ItemEquipSlot, ItemName, WorldItem};
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct CharacterWeapon(pub Option<Entity>);

#[derive(Component, Default, Debug)]
pub struct CharacterArmor(pub Option<Entity>);

pub struct EquipItem {
    pub item: Entity,
    pub character: Entity,
}
impl Command for EquipItem {
    fn apply(self, world: &mut World) {
        let item = world.entity(self.item);
        let Some(slot) = item.get::<ItemEquipSlot>().cloned() else {
            warn!(
                "Cannot equip item {} because it does not have defined slot",
                item.get::<ItemName>()
                    .map(|i| i.0.as_str())
                    .unwrap_or("<???>")
            );
            return;
        };

        let mut character = world.entity_mut(self.character);
        match slot {
            ItemEquipSlot::Weapon => {
                character.get_mut::<CharacterWeapon>().unwrap().0 = Some(self.item)
            }
            ItemEquipSlot::Armor => {
                character.get_mut::<CharacterArmor>().unwrap().0 = Some(self.item)
            }
        }
    }
}

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
        // todo: unequip item

        let mut item = world.entity_mut(self.item);
        item.insert(WorldItem);
        *item.get_mut::<Visibility>().unwrap() = Visibility::Inherited;
    }
}
