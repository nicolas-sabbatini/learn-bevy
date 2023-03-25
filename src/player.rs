use bevy::prelude::*;

use crate::{
    commons::{EntitySize, Rotation, Velocity},
    system_sets::GameSet,
};

const PLAYER_SPAWN_XYZ: (f32, f32, f32) = (0.0, 0.0, 100.0);
const PLAYER_DRAG_FACTOR: f32 = 0.1;
const PLAYER_SPEED: f32 = 250.0;
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.67, 1.0);
const PLAYER_SPRITE_SIZE: f32 = 64.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(handle_player_input.in_set(GameSet::InputHandle));
    }
}

#[derive(Debug, Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    tag: Player,
    name: Name,
    velocity: Velocity,
    rotation: Rotation,
    entity_size: EntitySize,
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        tag: Player,
        name: Name::new("Player Ship"),
        velocity: Velocity(Vec2::ZERO),
        rotation: Rotation(0.0),
        entity_size: EntitySize(PLAYER_SPRITE_SIZE),
        sprite: SpriteBundle {
            texture: asset_server.load("sprites/ship_E.png"),
            transform: Transform::from_xyz(
                PLAYER_SPAWN_XYZ.0,
                PLAYER_SPAWN_XYZ.1,
                PLAYER_SPAWN_XYZ.2,
            ),
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        },
    });
}

fn handle_player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
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

    let mut velocity = query
        .get_single_mut()
        .expect("Can't get player, in move player system");

    let stearing_vector = target_speed - velocity.0;
    velocity.0 += stearing_vector * PLAYER_DRAG_FACTOR;
}
