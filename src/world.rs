use bevy::prelude::*;
use bevy_atmosphere::{
    collection::nishita::Nishita, model::AtmosphereModel, system_param::AtmosphereMut,
};
use bevy_rapier3d::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
            //     .insert_resource(CycleTimer(Timer::new(
            //         bevy::utils::Duration::from_millis(5000), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            //         TimerMode::Repeating,
            //     )))
            .add_systems(Startup, (spawn_light, spawn_floor));
        // .add_systems(Update, daylight_cycle);
    }
}

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
struct Sun;

// // Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
// #[derive(Resource)]
// struct CycleTimer(Timer);

// fn daylight_cycle(
//     mut atmosphere: AtmosphereMut<Nishita>,
//     mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
//     mut timer: ResMut<CycleTimer>,
//     time: Res<Time>,
// ) {
//     timer.0.tick(time.delta());

//     if timer.0.finished() {
//         let t = time.elapsed_seconds_wrapped() / 2.0;
//         atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

//         if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
//             light_trans.rotation = Quat::from_rotation_x(-t);
//             directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
//         }
//     }
// }

fn spawn_light(mut commands: Commands) {
    // let mut transform = Transform::default();
    // transform.rotate(Quat::from_rotation_x(5.0));
    // commands.spawn((
    //     DirectionalLightBundle {
    //         transform,
    //         ..Default::default()
    //     },
    //     Sun, // Marks the light as Sun
    // ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..Default::default()
    });
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(shape::Plane::from_size(100.).into()),
    //         material: materials.add(Color::DARK_GRAY.into()),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(Collider::cuboid(100.0, 0.1, 100.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 0., 0.0)));

    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(PbrBundle {
    //         mesh: meshes.add(shape::Capsule::default().into()),
    //         material: materials.add(Color::ALICE_BLUE.into()),
    //         ..Default::default()
    //     })
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
    let mut mesh = Mesh::from(shape::Plane {
        size: 100.,
        subdivisions: 100,
    });

    if let Some(pos) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        if let Some(position_array) = pos.as_float3() {
            let mut position_vec = position_array.to_vec();
            let mut colors_vec = Vec::new();

            let noise = Perlin::new(8);
            let scale = 0.001;

            for pos in position_vec.iter_mut() {
                let height = noise.get([pos[0] as f64 * scale, pos[1] as f64 * scale]) as f32;
                pos[1] = height;

                let color_code = ((height * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
                let color = Color::rgb_u8(color_code, color_code, color_code);
                let color_linear_f32 = color.as_linear_rgba_f32();
                colors_vec.push(color_linear_f32);
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position_vec);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors_vec);
        }
    }

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            ..Default::default()
        })
        .insert(Collider::cuboid(50., 0.1, 50.));
}
