use bevy::{prelude::*, window::PrimaryWindow};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: assert_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    for key in keyboard_input.get_pressed() {
        #[rustfmt::skip] // To line up the key codes
        match key {
            KeyCode::Left  | KeyCode::A => direction.x -= 1.0,
            KeyCode::Right | KeyCode::D => direction.x += 1.0,
            KeyCode::Up    | KeyCode::W => direction.y += 1.0,
            KeyCode::Down  | KeyCode::S => direction.y -= 1.0,
            _ => {}
        };
    }

    direction = direction.normalize_or_zero();

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    // for mut transform in query.iter_mut() {
    //     if keyboard_input.pressed(KeyCode::Left) {
    //         transform.translation.x -= 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::Right) {
    //         transform.translation.x += 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::Up) {
    //         transform.translation.y += 1.0;
    //     }
    //     if keyboard_input.pressed(KeyCode::Down) {
    //         transform.translation.y -= 1.0;
    //     }
    // }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else { 
        return;
    };

    let window = window_query.single();
    let half_player_size = PLAYER_SIZE / 2.0;

    // Calculate the min and max x and y values that the player can be at
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    // Cap the player's x and y values
    let x_capped = transform.translation.x.min(x_max).max(x_min);
    let y_capped = transform.translation.y.min(y_max).max(y_min);

    // Update the player's position with the capped values
    transform.translation.x = x_capped;
    transform.translation.y = y_capped;
}
