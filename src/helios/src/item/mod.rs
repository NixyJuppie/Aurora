use bevy::prelude::*;

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
