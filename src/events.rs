use bevy::prelude::Entity;

pub struct HitCreatureEvent {
    target: Entity,
    physical_damage: i16,
    magical_damage: i16,
}
