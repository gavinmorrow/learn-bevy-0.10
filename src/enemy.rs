use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::random_spawn;

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

#[derive(Component, Clone, Copy)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Generate an array of enemies
    let mut enemies = [Enemy {
        direction: Vec2::ZERO,
    }; NUM_ENEMIES];

    // Give each one a random direction
    for enemy in enemies.iter_mut() {
        enemy.direction = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize();
    }

    random_spawn::spawn(
        commands,
        window_query,
        asset_server,
        "sprites/ball_red_large.png",
        enemies,
        SIZE,
    );
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
// oh and ig the for loop
//
// Also, theoretically, this should not be necessary, as the `update_direction` system should take
// care of it. It doesn't because it's possible for the ordering to go:
// `update_direction` -> `r#move` -> (next frame) `r#move` -> `update_direction`
pub fn confine_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in enemy_query.iter_mut() {
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
}
