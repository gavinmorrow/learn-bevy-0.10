use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

// FIXME: still requires a lot of boilerplate
// maybe use a macro instead?
pub fn spawn<const N: usize, T: Component>(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    texture_asset_path: &'static str,
    sprites: [T; N],
    size: f32,
) {
    let window = window_query.single();

    for sprite in sprites {
        let (x, y) = gen_random_pos(window.width(), window.height(), size);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load(texture_asset_path),
                ..default()
            },
            sprite,
        ));
    }
}

/// Generate a random position for the sprite.
///
/// # Arguments
///
/// * `width` - The width of the window.
/// * `height` - The height of the window.
///
/// # Returns
///
/// A tuple containing the x and y coordinates of the sprite (in that order).
fn gen_random_pos(width: f32, height: f32, size: f32) -> (f32, f32) {
    // Generate a random position for the sprite
    //
    // The buffer ensures that the sprite is spawned at least half its size away from the edge
    // of the screen (ie it will always spawn fully on-screen).
    let buffer = size / 2.0;
    let x = buffer + random::<f32>() * (width - size);
    let y = buffer + random::<f32>() * (height - size);

    (x, y)
}
