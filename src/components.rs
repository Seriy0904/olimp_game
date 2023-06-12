use bevy::prelude::*;

#[derive(Component)]
pub struct Player();
#[derive(Component)]
pub struct Creature {
    pub name: String,
    pub hp: i16,
    pub mana: i16,
    pub speed: f32,
} 