use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;
use super::components::*;
use crate::{PLAYER_SIZE,ENEMY_SIZE};


pub fn spawn_one_enemy(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (Enemy(),
        Creature {
            name: String::from("Ghost"),
            hp: 50,
            mana: 10,
            speed: 4.0,
        },
        SpriteBundle {
            texture: asset_server.load("enemy1.png"),
            transform: Transform::from_xyz(random::<f32>()*(window.width()-ENEMY_SIZE)+ENEMY_SIZE/2.0,random::<f32>()*(window.height()-ENEMY_SIZE)+ENEMY_SIZE/2.0,0.0),
            ..default()
        })
    );
}
pub fn enemy_to_player(
    mut enemy_query: Query<(&mut Transform,&Creature), (With<Enemy>,Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time:Res<Time>
){
    let player_translation = &player_query.get_single().unwrap().translation; 
    for (mut enemy_transform,creature) in enemy_query.iter_mut(){
        if enemy_transform.translation.distance(*player_translation)<= PLAYER_SIZE/2.0+ENEMY_SIZE/2.0{
            //TODO: do smth when they are conflict, somehow call method "hit()"
        }
        let mut distance = *player_translation - enemy_transform.translation;
        distance = distance.normalize();
        enemy_transform.translation += ENEMY_SIZE*creature.speed*time.delta_seconds()*distance;
    }
}
