use bevy::{prelude::*, window::PrimaryWindow};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player;

pub fn spawn(
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

pub fn r#move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else { return; };

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

    // Normalize the direction vector so that diagonal movement isn't faster
    direction = direction.normalize_or_zero();

    // Modify the player's position based on the direction vector
    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}

pub fn confine_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else { return; };

    // Get window dimensions
    let window = window_query.single();
    let window_size = [window.width(), window.height()];

    // Calculate the min and max x and y values that the player can be at
    let [x_cap, y_cap] = crate::cap::calc_cap(window_size, [PLAYER_SIZE, PLAYER_SIZE]);

    // Cap the player's x and y values
    let x_capped = x_cap.apply(transform.translation.x);
    let y_capped = y_cap.apply(transform.translation.y);

    // Update the player's position with the capped values
    transform.translation.x = x_capped;
    transform.translation.y = y_capped;
}
