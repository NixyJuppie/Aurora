#![allow(clippy::type_complexity)]

pub use bevy;
pub use bevy_rapier3d;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CollisionGroups, Group, RapierDebugRenderPlugin, RapierPhysicsPlugin,
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use bitflags::bitflags;

use camera::CameraPlugin;
use character::CharacterPlugin;
use debug_ui::DebugUiPlugin;
use input::InputPlugin;
use player::PlayerPlugin;

pub mod camera;
pub mod character;
pub mod input;
pub mod item;

mod debug_ui;
pub mod player;

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
        let screen_diagnostics = ScreenDiagnosticsPlugin {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        };

        PluginGroupBuilder::start::<Self>()
            .add(RapierDebugRenderPlugin::default())
            .add(screen_diagnostics)
            .add(ScreenFrameDiagnosticsPlugin)
            .add(ScreenEntityDiagnosticsPlugin)
            .add(DebugUiPlugin)
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct HeliosCollision: u32 {
        const WORLD = 1 << 0;
        const CHARACTER = 1 << 1;
        const ITEM = 1 << 2;
        const CONTAINER = 1 << 3;

        const WORLD_FILTER = Self::WORLD.bits() | Self::CHARACTER.bits() | Self::ITEM.bits() | Self::CONTAINER.bits();
        const CHARACTER_FILTER = Self::WORLD.bits() | Self::CHARACTER.bits() | Self::CONTAINER.bits();
        const ITEM_FILTER = Self::WORLD.bits() | Self::ITEM.bits() | Self::CONTAINER.bits();
        const CONTAINER_FILTER = Self::WORLD.bits() | Self::CHARACTER.bits() | Self::ITEM.bits() | Self::CONTAINER.bits();
    }
}

impl From<HeliosCollision> for Group {
    fn from(value: HeliosCollision) -> Self {
        Group::from_bits_retain(value.bits())
    }
}

impl HeliosCollision {
    pub fn none_groups() -> CollisionGroups {
        CollisionGroups::new(Group::NONE, Group::NONE)
    }

    pub fn world_groups() -> CollisionGroups {
        CollisionGroups::new(Self::WORLD.into(), Self::WORLD_FILTER.into())
    }
    pub fn world_only_groups() -> CollisionGroups {
        CollisionGroups::new(Self::WORLD.into(), Self::WORLD.into())
    }

    pub fn character_groups() -> CollisionGroups {
        CollisionGroups::new(Self::CHARACTER.into(), Self::CHARACTER_FILTER.into())
    }
    pub fn character_only_groups() -> CollisionGroups {
        CollisionGroups::new(Self::CHARACTER.into(), Self::CHARACTER.into())
    }

    pub fn item_groups() -> CollisionGroups {
        CollisionGroups::new(Self::ITEM.into(), Self::ITEM_FILTER.into())
    }
    pub fn item_only_groups() -> CollisionGroups {
        CollisionGroups::new(Self::ITEM.into(), Self::ITEM.into())
    }

    pub fn container_groups() -> CollisionGroups {
        CollisionGroups::new(Self::CONTAINER.into(), Self::CONTAINER_FILTER.into())
    }
    pub fn container_only_groups() -> CollisionGroups {
        CollisionGroups::new(Self::CONTAINER.into(), Self::CONTAINER.into())
    }
}
