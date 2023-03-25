use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use commons::CommonsPlugin;
use config::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use system_sets::SystemSetPlugin;

// Load and use this module on debug
#[cfg(debug_assertions)]
mod debug_plugin;
#[cfg(debug_assertions)]
use debug_plugin::DebugPlugin;

mod camera;
mod commons;
mod config;
mod enemy;
mod player;
mod system_sets;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
            title: WIN_TITLE.to_string(),
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));

    // Add this plugins and system on debug
    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CommonsPlugin)
        .add_plugin(SystemSetPlugin);

    app.run();
}
