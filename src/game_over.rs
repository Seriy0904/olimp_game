use bevy::prelude::*;

use crate::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_game_over.in_schedule(OnEnter(AppState::MainMenu)));
    }
}
fn setup_game_over(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((UiCameraConfig::default()));
    commands.spawn(
        (ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}
