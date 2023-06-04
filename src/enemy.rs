use bevy::prelude::*;
use rand::random;

use crate::{
    commons::{EntitySize, Rotation, Velocity},
    config::{WIN_HEIGHT, WIN_WIDTH},
};

const ENEMY_COLOR: Color = Color::rgb(0.63, 0.41, 0.22);
const ENEMY_SPRITE_SIZE: f32 = 64.0;
const ENEMY_Z_POSITION: f32 = 10.0;
const ENEMY_SPEED: f32 = 200.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .add_system(enemy_on_enemy_colition);
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
    for i in 0..10 {
        let rand_x = (random::<f32>() * WIN_WIDTH) - (WIN_WIDTH / 2.0);
        let rand_y = (random::<f32>() * WIN_HEIGHT) - (WIN_HEIGHT / 2.0);
        let rand_velocity = Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0)
            .normalize()
            * ENEMY_SPEED;
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

fn enemy_on_enemy_colition(mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    let mut query = query.iter_combinations_mut();
    while let Some([(mut t1, mut v1), (mut t2, mut v2)]) = query.fetch_next() {
        let collition_vector = t1.translation.truncate() - t2.translation.truncate();

        if collition_vector.length() < (ENEMY_SPRITE_SIZE / 2.0) + 2.5 {
            let collition_vector_normalized = collition_vector.normalize();
            let v1_cvn = v1.0.dot(collition_vector_normalized);
            let v2_cvn = v2.0.dot(collition_vector_normalized);
            let new_vel_escalar = v1_cvn - v2_cvn;

            v1.0 -= collition_vector_normalized * new_vel_escalar;
            v2.0 += collition_vector_normalized * new_vel_escalar;
            t1.translation += collition_vector_normalized.extend(0.0);
            t2.translation -= collition_vector_normalized.extend(0.0);
        }
    }
}
