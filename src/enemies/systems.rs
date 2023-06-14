use super::components::*;
use super::events::*;
use crate::{ENEMY_SIZE, PLAYER_SIZE};
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub fn spawn_ghosts(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..1 {
        commands.spawn((
            Enemy,
            Creature {
                name: String::from("Ghost"),
                hp: 50,
                mana: 10,
                speed: rng.gen_range(0..10) as f32 * 0.1 + 3.0,
                auto_attack: AutoAttack {
                    auto_attack_damage: 5,
                    auto_attack_range: 10.0,
                    ..Default::default()
                },
            },
            Velocity {
                ..Default::default()
            },
            SpriteBundle {
                texture: asset_server.load("enemy1.png"),
                transform: Transform::from_xyz(
                    random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.0,
                    random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
            Collider {
                size: Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            },
        ));
    }
}
pub fn enemy_to_player(
    mut enemy_query: Query<(&Transform, &Creature, &mut Velocity), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_translation = player_query.get_single().unwrap().translation;
    for (enemy_transform, creature, mut enemy_velocity) in enemy_query.iter_mut() {
        let distance = (player_translation - enemy_transform.translation).normalize();
        let pred_speed = ENEMY_SIZE * creature.speed;
        enemy_velocity.direction_per_sec = distance * pred_speed;
    }
}
pub fn enemy_moving(
    mut enemy_query: Query<(&mut Transform, &mut Velocity), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_translation = player_query.single().translation;
    for (mut enemy_transform, mut enemy_velocity) in enemy_query.iter_mut() {
        let enemy_pred_translation =
            enemy_transform.translation + enemy_velocity.direction_per_sec * time.delta_seconds();
        if collide(
            enemy_pred_translation,
            Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            player_translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        )
        .is_none()
        {
            enemy_transform.translation = enemy_pred_translation;
            enemy_velocity.direction_per_sec = default();
        }
    }
}
pub fn enemy_auto_attack_target(
    mut enemy_query: Query<(&Transform, &mut Creature, &Collider), With<Enemy>>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
    mut physical_damage_event: EventWriter<HitCreaturePhysical>,
    time: Res<Time>,
) {
    let (player_entity, player_transform) = player_query.single();
    for (enemy_transform, mut enemy_creature, collider) in enemy_query.iter_mut() {
        let enemy_auto_attack_stats = &mut enemy_creature.auto_attack;
        if player_transform
            .translation
            .distance(enemy_transform.translation)-collider.diagonal()-PLAYER_SIZE/2.0
            <= enemy_auto_attack_stats.auto_attack_range
        {
            if enemy_auto_attack_stats.cool_down_left - time.delta_seconds() <= 0.0 {
                physical_damage_event.send(HitCreaturePhysical {
                    target: player_entity,
                    physical_damage: enemy_auto_attack_stats.auto_attack_damage,
                });
                enemy_auto_attack_stats.cool_down_left =
                    1.0 / enemy_auto_attack_stats.auto_attack_speed as f32;
            }
        }
        if !(enemy_auto_attack_stats.cool_down_left - time.delta_seconds() <= 0.0) {
            enemy_auto_attack_stats.cool_down_left -= time.delta_seconds();
        }
    }
}
