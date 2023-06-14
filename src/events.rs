use bevy::prelude::Entity;

pub struct HitCreaturePhysical {
    pub target: Entity,
    pub physical_damage: u16,
}
