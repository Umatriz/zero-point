use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn((
        camera,
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 50.0),
            mouse_sensitivity: 2.4,
            cursor_lock_key: KeyCode::Escape,
            ..Default::default()
        },
        AtmosphereCamera::default(),
    ));
}
