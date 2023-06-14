pub const ENEMY_SIZE: f32 = 32.0;
pub const PLAYER_SIZE: f32 = 32.0;

mod components;
mod debug;
mod enemies;
mod events;
mod player;

use bevy::{prelude::*, window::PrimaryWindow};
use components::{Creature, Player};
use debug::DebugPlugin;
use enemies::EnemiesPlugin;
use events::HitCreaturePhysical;
use player::PlayerPlugin;
fn main() {
    App::new()
        .add_event::<HitCreaturePhysical>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin())
        .add_plugin(EnemiesPlugin())
        .add_plugin(DebugPlugin)
        .add_startup_system(setup)
        .add_system(todo_ui)
        .run();
}
fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
fn todo_ui(player_query: Query<&Creature, With<Player>>){
    let player_creature = player_query.single();
    println!("HP: {}", player_creature.hp);
}