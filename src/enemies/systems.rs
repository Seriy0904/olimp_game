use super::components::*;
use super::events::*;
use crate::{ENEMY_SIZE, PLAYER_SIZE};
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub fn spawn_one_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..3 {
        commands.spawn((
            Enemy(),
            Creature {
                name: String::from("Ghost"),
                hp: 50,
                mana: 10,
                speed: 4.0,
                auto_attack_damage: 10,
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
        ));
    }
}
pub fn enemy_to_player(
    mut enemy_query: Query<(&Transform, &Creature, &mut Velocity), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_translation = player_query.get_single().unwrap().translation;
    for (enemy_transform, creature, mut enemy_velocity) in enemy_query.iter_mut() {
        if enemy_transform.translation.distance(player_translation)
            > PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0
        {
            let mut distance = player_translation - enemy_transform.translation;
            distance = distance.normalize();
            enemy_velocity.direction = distance;
            enemy_velocity.speed = ENEMY_SIZE * creature.speed * time.delta_seconds();
        }
    }
}
pub fn enemy_moving(mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = combinations.fetch_next() {
        if let Some(_) = collide(
            a1.0.translation+a1.1.speed*a1.1.direction,
            Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            a2.0.translation+a2.1.speed*a2.1.direction,
            Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
        ) {
            a1.1.direction=(a2.0.translation-a1.0.translation).normalize();
            a2.1.direction=(a1.0.translation-a2.0.translation).normalize();
            a1.1.speed=default();
            a1.1.direction=default();
            a2.1.speed=default();
            a2.1.direction=default();
        }
    }
    for (mut enemy_transform, mut enemy_velocity) in query.iter_mut() {
        enemy_transform.translation += enemy_velocity.speed * enemy_velocity.direction;
        enemy_velocity.speed = default();
        enemy_velocity.direction = default();
    }
}
