mod systems;
use bevy::prelude::*;
use systems::*;
use super::components;

pub struct EnemiesPlugin();
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_one_enemy).add_system(enemy_to_player);
    }
}