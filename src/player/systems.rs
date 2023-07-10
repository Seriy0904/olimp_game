use super::components::*;
use super::events::*;
use crate::{ PLAYER_SIZE};
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};

pub fn player_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Player,
        Creature {
            name: String::from("FirstCharacter"),
            hp: 100,
            mana: 10,
            speed: 5.0,
            auto_attack: AutoAttack { auto_attack_damage: 10, auto_attack_range: 10.0,..Default::default() },
        },
        SpriteBundle {
            texture: asset_server.load("frst_character.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Velocity {
            ..Default::default()
        },
    ));
}

pub fn player_moving_keyboard(
    kbd: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &Creature), With<Player>>,
) {
    let (mut player_velocity, creature_stats) = player_query.get_single_mut().unwrap();
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
    direction = direction.normalize_or_zero();
    if kbd.pressed(KeyCode::LShift) {
        direction *= 1.5;
    }
    player_velocity.direction_per_sec =
        direction * PLAYER_SIZE * creature_stats.speed;
}
pub fn player_moving_bordering(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    colliders_query: Query<(&Transform, &Collider),Without<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let player = &mut player_query.get_single_mut().unwrap();
    let player_translation = &mut player.0.translation;
    let mut player_pred_translation = *player_translation + player.1.direction_per_sec*time.delta_seconds();
    let player_half = PLAYER_SIZE / 2.0;
    let x_min = player_half;
    let x_max = window.width() - player_half;
    let y_min = player_half;
    let y_max = window.height() - player_half;
    //X bordering
    if player_pred_translation.x < x_min {
        player_pred_translation.x = x_min;
    } else if player_pred_translation.x > x_max {
        player_pred_translation.x = x_max;
    }
    //Y bordering
    if player_pred_translation.y < y_min {
        player_pred_translation.y = y_min;
    } else if player_pred_translation.y > y_max {
        player_pred_translation.y = y_max;
    }
    for (collider_transform, collider) in colliders_query.iter() {
        if collide(
            Vec3::new(player_pred_translation.x, player_translation.y, 0.0),
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            collider_transform.translation,
            collider.size,
        )
        .is_some()
        { 
            player_pred_translation.x=player_translation.x;
        }
        if collide(
            Vec3::new(player_translation.x, player_pred_translation.y, 0.0),
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            collider_transform.translation,
            collider.size,
        )
        .is_some()
        { 
            player_pred_translation.y=player_translation.y;
        }
    }
    *player_translation = player_pred_translation;
}
pub fn player_get_physical_hit(mut player_query: Query<(&mut Creature, Entity), With<Player>>, mut physical_hit_event: EventReader<HitCreaturePhysical>){
    let (mut player_creature, player_entity) = player_query.single_mut();
    for event in physical_hit_event.iter(){
        if event.target == player_entity{
            player_creature.hp-=event.physical_damage;
        }
    }
}
