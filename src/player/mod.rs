mod systems;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

use super::components;
use super::events;
pub struct PlayerPlugin();
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_startup.in_schedule(OnEnter(AppState::MainGame))).add_systems(
            (
                player_moving_keyboard,
                player_moving_bordering.after(player_moving_keyboard),
                player_get_physical_hit,
            )
                .in_set(OnUpdate(AppState::MainGame)),
        );
    }
}
