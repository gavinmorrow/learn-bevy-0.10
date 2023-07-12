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

pub fn confine_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else { 
        return;
    };

	// Get window dimensions
    let window = window_query.single();
	let width = window.width();
	let height = window.height();

    // Calculate the min and max x and y values that the player can be at
    let (x_cap, y_cap) = calc_cap(width, height, PLAYER_SIZE);

    // Cap the player's x and y values
    let x_capped = cap_value(transform.translation.x, x_cap);
    let y_capped = cap_value(transform.translation.y, y_cap);

    // Update the player's position with the capped values
    transform.translation.x = x_capped;
    transform.translation.y = y_capped;
}

fn cap_value(value: f32, (min, max): (f32, f32)) -> f32 {
	value.min(max).max(min)
}

fn calc_cap(width: f32, height: f32, size: f32) -> ((f32, f32), (f32, f32)) {
	let half_size = size / 2.0;
	
	let x_min = 0.0 + half_size;
    let x_max = width - half_size;
    let y_min = 0.0 + half_size;
    let y_max = height - half_size;

	((x_min, x_max), (y_min, y_max))
}
