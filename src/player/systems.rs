use super::components::*;
use bevy::{prelude::*, window::PrimaryWindow};
use crate::{PLAYER_SIZE,ENEMY_SIZE};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Player(),
        Creature {
            name: String::from("FirstCharacter"),
            hp: 100,
            mana: 10,
            speed: 5.0,
        },
        SpriteBundle {
            texture: asset_server.load("frst_character.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
    ));
}

pub fn player_moving(
    kbd: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Creature), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, creature_stats) = player_query.get_single_mut().unwrap();
    let mut direction = Vec3::ZERO;
    //Moving direction creating
    let (up, down, left, right) = (
        kbd.pressed(KeyCode::Up) || kbd.pressed(KeyCode::W),
        kbd.pressed(KeyCode::Down) || kbd.pressed(KeyCode::S),
        kbd.pressed(KeyCode::Left) || kbd.pressed(KeyCode::A),
        kbd.pressed(KeyCode::Right) || kbd.pressed(KeyCode::D),
    );
    if up {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if down {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if right {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if left {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if kbd.pressed(KeyCode::LShift) {
        direction *= 1.5;
    }
    player_transform.translation +=
        direction * PLAYER_SIZE * creature_stats.speed * time.delta_seconds();
}
pub fn player_moving_bordering(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut player_translation = &mut player_query.get_single_mut().unwrap().translation;
    let player_half = PLAYER_SIZE / 2.0;
    let x_min = player_half;
    let x_max = window.width() - player_half;
    let y_min = player_half;
    let y_max = window.height() - player_half;
    //X bordering
    if player_translation.x < x_min {
        player_translation.x = x_min;
    } else if player_translation.x > x_max {
        player_translation.x = x_max;
    }
    //Y bordering
    if player_translation.y < y_min {
        player_translation.y = y_min;
    } else if player_translation.y > y_max {
        player_translation.y = y_max;
    }
}
