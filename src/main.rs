// use bevy::asset::AssetMetaCheck;
use bevy::{prelude::*, window::WindowMode};
use mandelbrot_shader::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mandelbrot".to_string(),
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
