mod systems;
use bevy::{prelude::*};
use systems::*;
use super::components;
use super::events;
pub struct PlayerPlugin();
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(startup)
        .add_system(player_moving_keyboard)
        .add_system(player_moving_bordering.after(player_moving_keyboard))
        .add_system(player_get_physical_hit);
    }
}
