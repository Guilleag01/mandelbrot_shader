use bevy::{
    prelude::{Component, Deref, Handle, Image, Query, Resource},
    render::{
        extract_resource::ExtractResource,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat::Rgba8Unorm, TextureUsages},
    },
};

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct MandelbrotImage(pub Handle<Image>);

pub fn create_image(width: u32, height: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    image
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::NoUninit)]
pub struct RenderParams {
    pub width_pixels: f64,
    pub height_pixels: f64,
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub iters: f64,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            width_pixels: Default::default(),
            height_pixels: Default::default(),
            width: Default::default(),
            height: 1.0,
            offset_x: Default::default(),
            offset_y: Default::default(),
            iters: Default::default(),
        }
    }
}

#[derive(Resource, Clone, Default, Component, ExtractResource)]
pub struct RenderParamsResource {
    pub render_params: RenderParams,
}

// pub fn update_params(
//     mut query: Query<&mut RenderParamsResource>,
//     // Otros parámetros necesarios
// ) {
//     for mut resource in query.iter_mut() {
//         // Actualiza los valores de los uniformes aquí
//         resource.render_params.width_pixels = 1.0;
//         resource.render_params.height_pixels = 2.0;
//         resource.render_params.width = 3.0;
//         resource.render_params.height = 4.0;
//         resource.render_params.offset_x = 5.0;
//         resource.render_params.offset_y = 6.0;
//         resource.render_params.iters = 7.0;
//     }
// }
