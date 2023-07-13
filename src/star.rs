use bevy::{prelude::*, window::PrimaryWindow};

use crate::random_spawn;

const NUM_STARS: usize = 10;
const SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Star;

pub fn spawn(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    random_spawn::spawn(
        commands,
        window_query,
        asset_server,
        "sprites/star.png",
        [Star {}; NUM_STARS],
        SIZE,
    );
}
