mod systems;
use bevy::prelude::*;
use systems::*;
use super::components;
use super::events;

pub struct EnemiesPlugin();
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_ghosts)
        .add_system(enemy_to_player)
        .add_system(enemy_moving)
        .add_system(enemy_auto_attack_target);
    }
}
