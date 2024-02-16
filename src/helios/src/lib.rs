#![allow(clippy::type_complexity)]
pub use bevy;
pub use bevy_rapier3d;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{RapierDebugRenderPlugin, RapierPhysicsPlugin};
use camera::CameraPlugin;
use character::player::PlayerPlugin;
use input::InputPlugin;

pub mod camera;
pub mod character;
pub mod input;

pub struct HeliosPlugins;
impl PluginGroup for HeliosPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<()>::default())
            .add(RapierDebugRenderPlugin::default())
            .add(InputPlugin)
            .add(CameraPlugin)
            .add(PlayerPlugin)
            .add(WorldInspectorPlugin::new())
    }
}
