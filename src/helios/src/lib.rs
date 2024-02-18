#![allow(clippy::type_complexity)]
pub use bevy;
pub use bevy_rapier3d;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CollisionGroups, Group, RapierDebugRenderPlugin, RapierPhysicsPlugin,
};
use bitflags::bitflags;

use camera::CameraPlugin;
use character::player::PlayerPlugin;
use character::CharacterPlugin;
use debug_ui::DebugUiPlugin;
use input::InputPlugin;

pub mod camera;
pub mod character;
pub mod input;
pub mod item;

mod debug_ui;

pub struct HeliosPlugins;
impl PluginGroup for HeliosPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<()>::default())
            .add(CharacterPlugin)
            .add(PlayerPlugin)
            .add(InputPlugin)
            .add(CameraPlugin)
    }
}

pub struct HeliosDebugPlugins;
impl PluginGroup for HeliosDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierDebugRenderPlugin::default())
            .add(DebugUiPlugin)
    }
}

bitflags! {
    pub struct HeliosCollision: u32 {
        const WORLD = 1 << 0;
        const CHARACTER = 1 << 1;

        const WORLD_FILTER = Self::WORLD.bits() | Self::CHARACTER.bits();
        const CHARACTER_FILTER = Self::WORLD.bits() | Self::CHARACTER.bits();
    }
}

impl From<HeliosCollision> for Group {
    fn from(value: HeliosCollision) -> Self {
        Group::from_bits_retain(value.bits())
    }
}

impl HeliosCollision {
    pub fn world_groups() -> CollisionGroups {
        CollisionGroups::new(Self::WORLD.into(), Self::WORLD_FILTER.into())
    }

    pub fn character_groups() -> CollisionGroups {
        CollisionGroups::new(Self::CHARACTER.into(), Self::CHARACTER_FILTER.into())
    }
}
