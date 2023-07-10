mod systems;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

use super::components;
use super::events;

pub struct EnemiesPlugin();
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ghosts.in_schedule(OnEnter(AppState::MainGame))).add_systems(
            (enemy_to_player, enemy_moving, enemy_auto_attack_target)
                .in_set(OnUpdate(AppState::MainGame)),
        );
    }
}
