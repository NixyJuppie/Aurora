#![allow(clippy::type_complexity)]
pub use bevy;
pub use bevy_rapier3d;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::debug_ui::DebugUiPlugin;
use camera::CameraPlugin;
use character::player::PlayerPlugin;
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
            .add(InputPlugin)
            .add(CameraPlugin)
            .add(PlayerPlugin)
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
