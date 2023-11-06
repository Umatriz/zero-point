use bevy::prelude::*;

pub mod gen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zero-Point".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run()
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn movement(keys: Res<Input<KeyCode>>, mut players: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 10.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 10.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 10.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 10.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.20;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in &mut players {
        transform.translation += move_delta;
    }
}
