use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

const NUM_STARS: usize = 10;
const SIZE: f32 = 64.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

#[derive(Component)]
pub struct Star;

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    for _ in 0..NUM_STARS {
        let (x, y) = gen_random_pos(window.width(), window.height());

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

/// Generate a random position for the star.
///
/// # Arguments
///
/// * `width` - The width of the window.
/// * `height` - The height of the window.
///
/// # Returns
///
/// A tuple containing the x and y coordinates of the star (in that order).
fn gen_random_pos(width: f32, height: f32) -> (f32, f32) {
    // Generate a random position for the star
    //
    // The buffer ensures that the star is spawned at least half its size away from the edge
    // of the screen.
    let buffer = SIZE / 2.0;
    let x = buffer + random::<f32>() * (width - SIZE);
    let y = buffer + random::<f32>() * (height - SIZE);

    (x, y)
}
