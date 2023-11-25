use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

use camera::CameraPlugin;
use gen::MapPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub mod gen;
pub mod logic;
pub mod utils;

pub mod camera;
pub mod player;
pub mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zero-Point".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(AtmospherePlugin)
        .add_plugins((WorldPlugin, CameraPlugin, PlayerPlugin, MapPlugin))
        .run()
}

// let lerp = |t: f64, a: DVec2, b: DVec2| a + t * (b - a);
