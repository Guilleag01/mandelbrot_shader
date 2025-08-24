// use bevy::asset::AssetMetaCheck;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use mandelbrot_shader::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mandelbrot".to_string(),
                resizable: true,
                // mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(1920_f32, 1200_f32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
