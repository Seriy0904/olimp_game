mod systems;
use bevy::{prelude::*};
use systems::*;
use super::components;
pub struct PlayerPlugin();
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(startup)
        .add_system(player_moving)
        .add_system(player_moving_bordering);
    }
}
