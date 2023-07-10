use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Creature {
    pub name: String,
    pub hp: u16,
    pub mana: u16,
    pub speed: f32,
    pub auto_attack: AutoAttack
}
pub struct AutoAttack{
    pub auto_attack_damage: u16,
    pub auto_attack_range: f32,
    pub auto_attack_speed: u8,
    pub auto_attack_target: Option<Entity>,
    pub cool_down_left: f32
}

#[derive(Component)]
pub struct Velocity {
    pub direction_per_sec: Vec3,
}
#[derive(Component)]
pub struct Collider {
    pub size: Vec2
}

impl Default for Creature {
    fn default() -> Self {
        Creature {
            name: "NULL".to_string(),
            hp: 0,
            mana: 0,
            speed: 0.0,
            auto_attack:default(),
        }
    }
}
impl Default for AutoAttack{
    fn default() -> Self {
        AutoAttack { auto_attack_damage: 0, auto_attack_range: 0.0, auto_attack_target: None,cool_down_left: 0.0, auto_attack_speed:1 }
    }
}
impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            direction_per_sec: Vec3::ZERO,
        }
    }
}
impl Collider{
    pub fn diagonal(&self)->f32{
        ((self.size.x*self.size.x)+(self.size.y+self.size.y)).sqrt()/2.0
    }
}