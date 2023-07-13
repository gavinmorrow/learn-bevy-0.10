use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const NUM_ENEMIES: usize = 4;
pub const SIZE: f32 = 64.0;
pub const SPEED: f32 = 200.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn)
            .add_system(r#move)
            .add_system(update_direction)
            .add_system(confine_movement);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    for _ in 0..NUM_ENEMIES {
        let (x, y) = gen_random_pos(window.width(), window.height());

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}

/// Generate a random position for the enemy.
///
/// # Arguments
///
/// * `width` - The width of the window.
/// * `height` - The height of the window.
///
/// # Returns
///
/// A tuple containing the x and y coordinates of the enemy (in that order).
fn gen_random_pos(width: f32, height: f32) -> (f32, f32) {
    // Generate a random position for the enemy
    //
    // The buffer ensures that the enemy is spawned at least half its size away from the edge
    // of the screen.
    //
    // Otherwise, it could spawn halfway off the screen, causing it to get stuck there (due to
    // the `update_direction` system).
    //
    // Technically the buffer code could be removed (the `confine_movement` system would take care
    // of it), but then the random generation is biased towards spawning at the edge.
    let buffer = SIZE / 2.0;
    let x = buffer + random::<f32>() * (width - SIZE);
    let y = buffer + random::<f32>() * (height - SIZE);

    (x, y)
}

pub fn r#move(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction.extend(0.0) * SPEED * time.delta_seconds();
    }
}

pub fn update_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();
    let window_size = [window.width(), window.height()];

    // Calculate the min and max x and y values that the player can be at
    let [x_cap, y_cap] = crate::cap::calc_cap(window_size, [SIZE, SIZE]);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        // The dx and dy are stored in variables so that we can test if a sound needs to be played
        // FIXME: is this really any less hacky than the tutorial guy's solution? (a `direction_changed` bool)

        let dx = if x_cap.test(translation.x) != std::cmp::Ordering::Equal {
            -1.0
        } else {
            1.0
        };

        let dy = if y_cap.test(translation.y) != std::cmp::Ordering::Equal {
            -1.0
        } else {
            1.0
        };

        enemy.direction.x *= dx;
        enemy.direction.y *= dy;

        if dx != 1.0 || dy != 1.0 {
            play_sound(&audio, &asset_server);
        }
    }
}

/// Play a random `pluck` sound.
fn play_sound(audio: &Res<Audio>, asset_server: &Res<AssetServer>) {
    // Play a random sound
    const SOUNDS: u32 = 2;
    let random_sound = random::<u32>() % SOUNDS;
    // FIXME: This is a bit of a hack, as it will break if the number goes above one digit
    let sound = asset_server.load(format!("audio/pluck_00{}.ogg", random_sound).as_str());
    audio.play(sound);
}

// FIXME: this is copy-pasted from the player's code
// (literally the only difference is the variable names and the type of the query)
pub fn confine_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = enemy_query.get_single_mut() else { return; };

    // Get window dimensions
    let window = window_query.single();
    let window_size = [window.width(), window.height()];

    // Calculate the min and max x and y values that the enemy can be at
    let [x_cap, y_cap] = crate::cap::calc_cap(window_size, [SIZE, SIZE]);

    // Cap the enemy's x and y values
    let x_capped = x_cap.apply(transform.translation.x);
    let y_capped = y_cap.apply(transform.translation.y);

    // Update the enemy's position with the capped values
    transform.translation.x = x_capped;
    transform.translation.y = y_capped;
}
