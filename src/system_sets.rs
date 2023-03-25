use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    InputHandle,
    Movement,
    CollisionDetection,
}

pub struct SystemSetPlugin;
impl Plugin for SystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(GameSet::InputHandle.before(GameSet::Movement))
            .configure_set(GameSet::Movement.before(GameSet::CollisionDetection));
    }
}
