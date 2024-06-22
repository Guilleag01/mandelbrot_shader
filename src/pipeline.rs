use std::borrow::Cow;

use bevy::{
    asset::AssetServer,
    prelude::{default, Commands, FromWorld, Image, Res, Resource},
    render::{
        render_asset::RenderAssets,
        render_graph,
        render_resource::{
            BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource,
            BindingType, BufferBindingType, BufferInitDescriptor, BufferUsages,
            CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor,
            ComputePipelineDescriptor, PipelineCache, ShaderStages, StorageTextureAccess,
            TextureFormat, TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};

use crate::{
    image::{MandelbrotImage, RenderParamsResource},
    SIMULATION_SIZE, WORKGROUP_SIZE,
};

#[derive(Resource)]
pub struct MandelbrotPipeline {
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
    texture_bind_group_layout: BindGroupLayout,
}

impl FromWorld for MandelbrotPipeline {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let texture_bind_group_layout = world.resource::<RenderDevice>().create_bind_group_layout(
            Some("Mandelbrot bind group layout"),
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );

        let pipeline_cache = world.resource::<PipelineCache>();

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/mandelbrot.wgsl");

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("Mandelbrot init pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: vec![],
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("Mandelbrot update pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: vec![],
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        // commands.insert_resource(RenderParamsResource::default());

        // let pipeline_handle = pipelines.add(pipeline_descriptor);

        MandelbrotPipeline {
            init_pipeline,
            update_pipeline,
            texture_bind_group_layout,
        }
    }
}

#[derive(Resource)]
struct MandelbrotBindGroup(pub BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<MandelbrotPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    mandelbrot_image: Res<MandelbrotImage>,
    params: Res<RenderParamsResource>,
) {
    // let view = &gpu_images[&mandelbrot_image.0];
    let view = gpu_images.get(mandelbrot_image.id()).unwrap();

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Render params buffer"),
        contents: bytemuck::cast_slice(&[params.render_params]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = render_device.create_bind_group(
        Some("Mandelbrot bind group"),
        &pipeline.texture_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: buffer.as_entire_binding(),
            },
        ],
    );

    commands.insert_resource(MandelbrotBindGroup(bind_group))
}

pub enum MandelbrotState {
    Loading,
    Init,
    Update,
}

pub struct MandelbrotNode {
    state: MandelbrotState,
}

impl Default for MandelbrotNode {
    fn default() -> Self {
        Self {
            state: MandelbrotState::Loading,
        }
    }
}

impl render_graph::Node for MandelbrotNode {
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let pipeline = world.resource::<MandelbrotPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            MandelbrotState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = MandelbrotState::Init;
                }
            }
            MandelbrotState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = MandelbrotState::Update;
                }
            }
            MandelbrotState::Update => {}
        }
    }

    fn run<'w>(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        world: &'w bevy::prelude::World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<MandelbrotBindGroup>().0;
        let pipeline = world.resource::<MandelbrotPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        match self.state {
            MandelbrotState::Loading => {}
            MandelbrotState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                )
            }
            MandelbrotState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                )
            }
        }

        Ok(())
    }
}
