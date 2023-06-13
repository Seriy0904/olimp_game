pub const ENEMY_SIZE: f32 = 32.0;
pub const PLAYER_SIZE: f32 = 32.0;

mod player;
mod enemies;
mod components;
mod events;

use bevy::{prelude::*, window::PrimaryWindow};
use enemies::EnemiesPlugin;
use player::PlayerPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin())
        .add_plugin(EnemiesPlugin())
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
