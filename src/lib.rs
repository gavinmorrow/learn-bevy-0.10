use bevy::{prelude::*, window::PrimaryWindow};
use enemy::Enemy;
use player::Player;

mod cap;
mod enemy;
mod player;
mod random_spawn;
mod star;

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(star::StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(check_player_enemy_collision)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}

pub fn check_player_enemy_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single() else {
        return;
    };

    for enemy in enemy_query.iter() {
        let distance = get_sprite_distance(player_transform, enemy);

        const THRESHOLD: f32 = (player::SIZE + enemy::SIZE) / 2.0;
        if distance <= THRESHOLD {
            // Touching
            println!("Game Over!");
            println!("Distance: {}", distance);
            println!("Player: {:?}", player_transform.translation);
            println!("Enemy: {:?}", enemy.translation);

            // Despawn player
            commands.entity(player_entity).despawn();

            // Play sound
            audio.play(asset_server.load("audio/explosionCrunch_000.ogg"));
        }
    }
}

fn get_sprite_distance(a: &Transform, b: &Transform) -> f32 {
    let a_pos = a.translation;
    let b_pos = b.translation;

    a_pos.distance(b_pos)
}
