#![allow(clippy::type_complexity)]

pub mod image;
pub mod pipeline;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_graph::{RenderGraph, RenderLabel};
use bevy::render::{Render, RenderSet};
use bevy::{app::App, render::RenderApp};
use image::{create_image, MandelbrotImage, RenderParams, RenderParamsResource};
use pipeline::{MandelbrotNode, MandelbrotPipeline};

const SIMULATION_SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;
// const DISPLAY_FACTOR: u32 = 4;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
}

pub struct GamePlugin;

#[derive(RenderLabel, Hash, Debug, Eq, PartialEq, Clone)]
struct MandelbrotLabel;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_params)
            // .add_systems(Update, update_res)
            .add_plugins(ExtractResourcePlugin::<MandelbrotImage>::default())
            .add_plugins(ExtractResourcePlugin::<RenderParamsResource>::default());

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(Render, pipeline::queue_bind_group.in_set(RenderSet::Queue));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(MandelbrotLabel, MandelbrotNode::default());
        render_graph.add_node_edge(MandelbrotLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app.init_resource::<MandelbrotPipeline>();
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, window: Query<&Window>) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();
    // let width = SIMULATION_SIZE.0;
    // let height = SIMULATION_SIZE.1;
    let image = create_image(width as u32, height as u32);
    let image = images.add(image.clone());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        texture: image.clone(),
        // transform: Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(RenderParamsResource {
        render_params: RenderParams {
            width_pixels: width as f64,   //SIMULATION_SIZE.0 as f64,
            height_pixels: height as f64, //SIMULATION_SIZE.1 as f64,
            width: 10_f64,
            height: height as f64 * 10_f64 / width as f64,
            offset_x: 0_f64,
            offset_y: 0_f64,
            iters: 100_f64,
        },
    });

    commands.insert_resource(MandelbrotImage(image));
}

fn update_params(
    mut params: ResMut<RenderParamsResource>,
    mut evr_scroll: EventReader<MouseWheel>,
    window: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line | MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    params.render_params.width *= 0.9;
                    params.render_params.height = params.render_params.height_pixels
                        * params.render_params.width
                        / params.render_params.width_pixels;
                    params.render_params.iters += 2_f64;
                    // if let Some(position) = window.cursor_position() {
                    //     let centered_pos_x =
                    //         position.x as f64 - params.render_params.width_pixels / 2_f64;
                    //     let centered_pos_y =
                    //         position.y as f64 - params.render_params.height_pixels / 2_f64;

                    //     let real_pos_x = centered_pos_x * params.render_params.width_pixels
                    //         / params.render_params.width;
                    //     let real_pos_y = centered_pos_y * params.render_params.height_pixels
                    //         / params.render_params.height;
                    //     params.render_params.offset_x += real_pos_x;
                    //     params.render_params.offset_y += real_pos_y;
                    //     // println!("Cursor is inside the primary window, at {:?}", position);
                    // }
                } else if ev.y < 0.0 {
                    params.render_params.width *= 1.1;
                    params.render_params.height = params.render_params.height_pixels
                        * params.render_params.width
                        / params.render_params.width_pixels;
                    params.render_params.iters -= 2_f64;
                    // if let Some(position) = window.cursor_position() {
                    //     let centered_pos_x =
                    //         position.x as f64 - params.render_params.width_pixels / 2_f64;
                    //     let centered_pos_y =
                    //         position.y as f64 - params.render_params.height_pixels / 2_f64;

                    //     let real_pos_x = centered_pos_x * params.render_params.width_pixels
                    //         / params.render_params.width;
                    //     let real_pos_y = centered_pos_y * params.render_params.height_pixels
                    //         / params.render_params.height;
                    //     params.render_params.offset_x += real_pos_x;
                    //     params.render_params.offset_y += real_pos_y;
                    //     // println!("Cursor is inside the primary window, at {:?}", position);
                    // }
                }
            }
        }
    }

    if buttons.pressed(MouseButton::Left) {
        for ev in evr_motion.read() {
            // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

            params.render_params.offset_x -=
                ev.delta.x as f64 * params.render_params.width / width as f64;
            params.render_params.offset_y -=
                ev.delta.y as f64 * params.render_params.height / height as f64;
        }
    }
}

// fn scroll_events(mut evr_scroll: EventReader<MouseWheel>) {
//     use bevy::input::mouse::MouseScrollUnit;
//     for ev in evr_scroll.read() {
//         match ev.unit {
//             MouseScrollUnit::Line => {
//                 println!(
//                     "Scroll (line units): vertical: {}, horizontal: {}",
//                     ev.y, ev.x
//                 );
//             }
//             MouseScrollUnit::Pixel => {
//                 println!(
//                     "Scroll (pixel units): vertical: {}, horizontal: {}",
//                     ev.y, ev.x
//                 );
//             }
//         }
//     }
// }

// fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
//     // Games typically only have one window (the primary window)
//     if let Some(position) = q_windows.single().cursor_position() {
//         println!("Cursor is inside the primary window, at {:?}", position);
//     } else {
//         println!("Cursor is not in the game window.");
//     }
// }
