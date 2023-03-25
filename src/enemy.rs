use bevy::prelude::*;
use rand::random;

use crate::{
    commons::{EntitySize, Rotation, Velocity},
    config::{WIN_HEIGHT, WIN_WIDTH},
};

const ENEMY_COLOR: Color = Color::rgb(0.63, 0.41, 0.22);
const ENEMY_SPRITE_SIZE: f32 = 64.0;
const ENEMY_Z_POSITION: f32 = 10.0;
const ENEMY_SPEED: f32 = 250.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

#[derive(Debug, Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    tag: Enemy,
    name: Name,
    velocity: Velocity,
    rotation: Rotation,
    entity_size: EntitySize,
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..5 {
        let rand_x = (random::<f32>() * WIN_WIDTH) - (WIN_WIDTH / 2.0);
        let rand_y = (random::<f32>() * WIN_HEIGHT) - (WIN_HEIGHT / 2.0);
        let rand_velocity = Vec2::new(random::<f32>(), random::<f32>()).normalize() * ENEMY_SPEED;
        commands.spawn(EnemyBundle {
            tag: Enemy,
            name: Name::new(format!("Enemy {i}")),
            velocity: Velocity(rand_velocity),
            rotation: Rotation(0.0),
            entity_size: EntitySize(ENEMY_SPRITE_SIZE),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/meteor_detailedLarge.png"),
                transform: Transform::from_xyz(rand_x, rand_y, ENEMY_Z_POSITION),
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    ..default()
                },
                ..default()
            },
        });
    }
}
