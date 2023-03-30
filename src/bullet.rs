use bevy::{
    prelude::{Bundle, Component, Name},
    sprite::SpriteBundle,
};

use crate::commons::{EntitySize, Rotation, Velocity};

#[derive(Debug, Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub tag: Bullet,
    pub name: Name,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub entity_size: EntitySize,
    #[bundle]
    pub sprite: SpriteBundle,
}
