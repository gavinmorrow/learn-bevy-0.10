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
            .add_system(update_direction);
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
    // the `confine_movement` system).
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
) {
    let window = window.single();
    let window_size = [window.width(), window.height()];

    // Calculate the min and max x and y values that the player can be at
    let [x_cap, y_cap] = crate::cap::calc_cap(window_size, [SIZE, SIZE]);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        if x_cap.test(translation.x) != std::cmp::Ordering::Equal {
            enemy.direction.x *= -1.0;
        }

        if y_cap.test(translation.y) != std::cmp::Ordering::Equal {
            enemy.direction.y *= -1.0;
        }
    }
}
