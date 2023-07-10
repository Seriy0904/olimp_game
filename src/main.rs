pub const ENEMY_SIZE: f32 = 32.0;
pub const PLAYER_SIZE: f32 = 32.0;

mod components;
mod debug;
mod enemies;
mod events;
mod player;
mod game_over;

use bevy::{prelude::*, window::PrimaryWindow};
use components::{Creature, Player, MainCamera};
use debug::DebugPlugin;
use enemies::EnemiesPlugin;
use events::HitCreaturePhysical;
use game_over::GameOverPlugin;
use player::PlayerPlugin;
    
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    MainGame,
    GameOver
}
fn main() {
    App::new()
    .add_startup_system(setup)
        .add_state::<AppState>()
        .add_event::<HitCreaturePhysical>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOverPlugin)
        .add_plugin(PlayerPlugin())
        .add_plugin(EnemiesPlugin())
        .add_plugin(DebugPlugin)
        .run();
}   
fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    },MainCamera));
}