use bevy::prelude::*;
use rand::random;

use crate::config::{WIN_HEIGHT, WIN_WIDTH};

const ENEMY_COLOR: Color = Color::rgb(0.63, 0.41, 0.22);
const ENEMY_SPRITE_SIZE: f32 = 64.0;
const ENEMY_Z_POSITION: f32 = 10.0;

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
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..5 {
        let rand_x = (random::<f32>() * WIN_WIDTH) - (WIN_WIDTH / 2.0);
        let rand_y = (random::<f32>() * WIN_HEIGHT) - (WIN_HEIGHT / 2.0);
        commands.spawn(EnemyBundle {
            tag: Enemy,
            name: Name::new(format!("Enemy {i}")),
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
