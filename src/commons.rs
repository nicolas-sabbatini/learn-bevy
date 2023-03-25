use bevy::prelude::*;

use crate::config::{WIN_HEIGHT, WIN_WIDTH};

// From where to calculate the rotation
const ROTATION_ANCHOR: Vec2 = Vec2::Y;

#[derive(Debug, Component)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Component)]
pub struct Rotation(pub f32);

#[derive(Debug, Component)]
pub struct EntitySize(pub f32);

pub struct CommonsPlugin;
impl Plugin for CommonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(warp_entity).add_system(move_entity);
    }
}

fn warp_entity(mut query: Query<(&mut Transform, &EntitySize)>) {
    for (mut pos, size) in &mut query {
        let half_entity_size = size.0 / 2.0;

        if pos.translation.x + half_entity_size < -(WIN_WIDTH / 2.0) {
            pos.translation.x = WIN_WIDTH / 2.0;
        } else if pos.translation.x - half_entity_size > WIN_WIDTH / 2.0 {
            pos.translation.x = -(WIN_WIDTH / 2.0);
        }

        if pos.translation.y + half_entity_size < -(WIN_HEIGHT / 2.0) {
            pos.translation.y = WIN_HEIGHT / 2.0;
        } else if pos.translation.y - half_entity_size > WIN_HEIGHT / 2.0 {
            pos.translation.y = -(WIN_HEIGHT / 2.0);
        }
    }
}

fn move_entity(mut query: Query<(&Velocity, &mut Transform, &mut Rotation)>, time: Res<Time>) {
    for (velocity, mut pos, mut rotation) in &mut query {
        pos.translation += (velocity.0 * time.delta_seconds()).extend(0.0);

        if velocity.0.length() != 0.0 {
            let angle = velocity.0.angle_between(ROTATION_ANCHOR);
            pos.rotate_z(rotation.0 - angle);
            rotation.0 = angle;
        }
    }
}