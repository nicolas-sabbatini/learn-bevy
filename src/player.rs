use bevy::prelude::*;

use crate::config::{WIN_HEIGHT, WIN_WIDTH};

const ROTATION_ANCHOR: Vec2 = Vec2::Y;

const PLAYER_START_XYZ: (f32, f32, f32) = (0.0, 0.0, 100.0);
const PLAYER_DRAG_FACTOR: f32 = 0.1;
const PLAYER_SPEED: f32 = 250.0;
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.67, 1.0);
const PLAYER_SPRITE_SIZE: f32 = 64.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(warp_player);
    }
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct Velocity(Vec2);

#[derive(Debug, Component)]
struct Rotation(f32);

#[derive(Bundle)]
struct PlayerBundle {
    tag: Player,
    name: Name,
    velocity: Velocity,
    rotation: Rotation,
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        tag: Player,
        name: Name::new("Player Ship"),
        velocity: Velocity(Vec2::ZERO),
        rotation: Rotation(0.0),
        sprite: SpriteBundle {
            texture: asset_server.load("sprites/ship_E.png"),
            transform: Transform::from_xyz(
                PLAYER_START_XYZ.0,
                PLAYER_START_XYZ.1,
                PLAYER_START_XYZ.2,
            ),
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        },
    });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Rotation), With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        direction += Vec2::new(-1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec2::new(1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec2::new(0.0, 1.0)
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += Vec2::new(0.0, -1.0)
    }
    let direction = direction.normalize_or_zero();
    let target_speed = direction * PLAYER_SPEED;

    let (mut velocity, mut pos, mut rotation) = query
        .get_single_mut()
        .expect("Can't get player, in move player system");

    let stearing_vector = target_speed - velocity.0;
    velocity.0 += stearing_vector * PLAYER_DRAG_FACTOR;
    pos.translation += (velocity.0 * time.delta_seconds()).extend(0.0);

    if velocity.0.length() != 0.0 {
        let angle = velocity.0.angle_between(ROTATION_ANCHOR);
        pos.rotate_z(rotation.0 - angle);
        rotation.0 = angle;
    }
}

fn warp_player(mut query: Query<&mut Transform, With<Player>>) {
    let mut pos = query
        .get_single_mut()
        .expect("Can't get player, in warp player system");

    let half_ship_size = PLAYER_SPRITE_SIZE / 2.0;

    if pos.translation.x + half_ship_size < -(WIN_WIDTH / 2.0) {
        pos.translation.x = WIN_WIDTH / 2.0;
    } else if pos.translation.x - half_ship_size > WIN_WIDTH / 2.0 {
        pos.translation.x = -(WIN_WIDTH / 2.0);
    }

    if pos.translation.y + half_ship_size < -(WIN_HEIGHT / 2.0) {
        pos.translation.y = WIN_HEIGHT / 2.0;
    } else if pos.translation.y - half_ship_size > WIN_HEIGHT / 2.0 {
        pos.translation.y = -(WIN_HEIGHT / 2.0);
    }
}
