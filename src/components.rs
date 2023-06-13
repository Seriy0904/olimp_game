use bevy::prelude::*;

#[derive(Component)]
pub struct Player();
#[derive(Component)]
pub struct Enemy();
#[derive(Component)]
pub struct Creature {
    pub name: String,
    pub hp: i16,
    pub mana: i16,
    pub speed: f32,
    pub auto_attack_damage: u16,
}
impl Default for Creature{
    fn default() -> Self {
        Creature { name: "NULL".to_string(), hp: 0, mana: 0, speed: 0.0, auto_attack_damage: 0 }
    }
}
#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
    pub direction: Vec3,
}
impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            speed: 0.0,
            direction: Vec3::ZERO,
        }
    }
}
