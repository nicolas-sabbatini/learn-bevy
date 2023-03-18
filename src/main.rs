// Load and use this module on debug
#[cfg(debug_assertions)]
use debug_plugin::DebugPlugin;
#[cfg(debug_assertions)]
mod debug_plugin;

use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use config::*;
use player::PlayerPlugin;

mod camera;
mod config;
mod player;

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

    app.add_plugin(CameraPlugin).add_plugin(PlayerPlugin);

    app.run();
}
