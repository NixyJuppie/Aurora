use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod player;

#[derive(Bundle, Default, Debug)]
pub struct CharacterBundle {
    pub collider: Collider,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
