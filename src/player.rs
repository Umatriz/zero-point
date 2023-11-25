use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let camera = camera_query.get_single().unwrap();

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += camera.forward();
        }
        if keys.pressed(KeyCode::A) {
            direction += camera.left();
        }
        if keys.pressed(KeyCode::S) {
            direction += camera.back();
        }
        if keys.pressed(KeyCode::D) {
            direction += camera.right();
        }
        // TODO: not working
        // if keys.pressed(KeyCode::Space) {
        //     direction.y += 50.;
        // }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    };

    commands
        .spawn((player, Player, MovementSpeed(2.0), ThirdPersonCameraTarget))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
