use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroU64;
use std::ops::Mul;

use super::cache::UniformBuffer;
use super::grass_pipeline::GrassPipeline;
use crate::bundle::WarblerHeight;
use crate::map::{NormalMap, YMap};
use crate::prelude::GrassColor;
use crate::{GrassConfiguration, GrassNoiseTexture};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages, TextureViewId,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::{FallbackImage, GpuImage};
use bytemuck::{Pod, Zeroable};
#[derive(Component)]
pub(crate) struct BindGroupBuffer<T> {
    pub bind_group: BindGroup,
    _inner: PhantomData<T>,
}
impl<T> BindGroupBuffer<T> {
    pub fn new(bind_group: BindGroup) -> Self {
        BindGroupBuffer {
            bind_group,
            _inner: PhantomData,
        }
    }
}
// #[derive(Component)]
// pub(crate) struct IndexBindgroup {
//     pub bind_group: BindGroup,
// }
// pub(crate) fn prepare_instance_index(
//     query: Query<(Entity, &PhaseViewEntity), With<GrassColor>>,
//     mut commands: Commands,
//     phases: Res<ViewBinnedRenderPhases<Opaque3d>>,

//     pipeline: Res<GrassPipeline>,
//     render_device: Res<RenderDevice>,
// ) {
//     for (e, view_e) in &query {
//         let phase = phases[&view_e.0];
//     }
//     for phase in phases.values().into_iter() {
//         phase.unbatchable_keys
//     }
//     for entity in &query {
//         let Some(item) = phases
//             .iter()
//             .flat_map(|phase| &phase.items)
//             .find(|item| item.entity == entity)
//         else {
//             continue;
//         };
//         let index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
//             label: Some("instance index buffer"),
//             contents: bytemuck::cast_slice(&[item.batch_range.start, 0, 0, 0]),
//             usage: BufferUsages::VERTEX | BufferUsages::UNIFORM | BufferUsages::COPY_DST,
//         });
//         let layout = &pipeline.instance_index_bind_group_layout;
//         let bind_group = render_device.create_bind_group(
//             "instance index bindgroup",
//             layout,
//             &BindGroupEntries::single(BindingResource::Buffer(BufferBinding {
//                 buffer: &index_buffer,
//                 offset: 0,
//                 size: None,
//             })),
//         );

//         commands
//             .entity(entity)
//             .insert(IndexBindgroup { bind_group });
//     }
// }
#[derive(Component)]
pub(crate) struct UniformHeightFlag;

pub(crate) fn prepare_height_buffer(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<GpuImage>>,

    render_device: Res<RenderDevice>,
    inserted_grass: Query<(Entity, &WarblerHeight)>,
) {
    for (entity, height) in inserted_grass.iter() {
        match height.clone() {
            WarblerHeight::Uniform(height) => {
                let layout = pipeline.uniform_height_layout.clone();

                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                    label: "grass blade height buffer".into(),
                    contents: bytemuck::bytes_of(&ShaderHeightUniform::from(height)),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::UNIFORM,
                });
                let bind_group = render_device.create_bind_group(
                    "grass blade height bind group",
                    &layout,
                    &BindGroupEntries::single(BindingResource::Buffer(BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: NonZeroU64::new(mem::size_of::<ShaderHeightUniform>() as u64),
                    })),
                );
                commands
                    .entity(entity)
                    .insert(BindGroupBuffer::<WarblerHeight>::new(bind_group))
                    .insert(UniformHeightFlag);
            }
            WarblerHeight::Texture(heights_texture) => {
                let layout = pipeline.heights_texture_layout.clone();

                let tex = if let Some(tex) = images.get(&heights_texture) {
                    &tex.texture_view
                } else {
                    &fallback_img.d2.texture_view
                };

                let bind_group = render_device.create_bind_group(
                    "grass height map bind group",
                    &layout,
                    &BindGroupEntries::single(BindingResource::TextureView(tex)),
                );
                commands
                    .entity(entity)
                    .insert(BindGroupBuffer::<WarblerHeight>::new(bind_group));
            }
        };
    }
}
pub(crate) fn prepare_grass_color(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    render_device: Res<RenderDevice>,
    inserted_grass: Query<(Entity, &GrassColor)>,
) {
    for (entity, color) in inserted_grass.iter() {
        let layout = pipeline.color_layout.clone();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: "grass color buffer".into(),
            contents: bytemuck::bytes_of(&ShaderColorUniform::from(color)),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });
        let bind_group = render_device.create_bind_group(
            "grass color bind group",
            &layout,
            &BindGroupEntries::single(BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: NonZeroU64::new(mem::size_of::<ShaderColorUniform>() as u64),
            })),
        );
        commands
            .entity(entity)
            .insert(BindGroupBuffer::<GrassColor>::new(bind_group));
    }
}

pub(crate) fn prepare_y_map_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<GrassPipeline>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<GpuImage>>,
    inserted_grass: Query<(Entity, &YMap, &Aabb)>,
) {
    let layout = pipeline.y_map_layout.clone();

    for (entity, y_map, aabb) in inserted_grass.iter() {
        let y_map_texture = if let Some(tex) = images.get(&y_map.y_map) {
            &tex.texture_view
        } else {
            &fallback_img.d2.texture_view
        };

        let shader_aabb = ShaderAabb::from(Vec3::from(aabb.half_extents.mul(2.)));
        let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("aabb buffer"),
            contents: bytemuck::bytes_of(&shader_aabb),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(
            "grass y-map bind group",
            &layout,
            &BindGroupEntries::sequential((
                BindingResource::TextureView(y_map_texture),
                BindingResource::Buffer(BufferBinding {
                    buffer: &aabb_buffer,
                    offset: 0,
                    size: None,
                }),
            )),
        );
        commands
            .entity(entity)
            .insert(BindGroupBuffer::<YMap>::new(bind_group));
    }
}
pub(crate) fn prepare_normal_map_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<GrassPipeline>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<GpuImage>>,
    inserted_grass: Query<(Entity, &NormalMap)>,
) {
    let layout = pipeline.normal_map_layout.clone();

    for (entity, normal_map) in inserted_grass.iter() {
        let normal_map_texture = if let Some(tex) = images.get(&normal_map.normal_map) {
            &tex.texture_view
        } else {
            &fallback_img.d2.texture_view
        };

        let bind_group = render_device.create_bind_group(
            "grass normal-map bind group",
            &layout,
            &BindGroupEntries::single(BindingResource::TextureView(normal_map_texture)),
        );
        commands
            .entity(entity)
            .insert(BindGroupBuffer::<NormalMap>::new(bind_group));
    }
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_uniform_buffers(
    pipeline: Res<GrassPipeline>,
    region_config: Res<GrassConfiguration>,
    noise_config: Res<GrassNoiseTexture>,
    fallback_img: Res<FallbackImage>,
    render_device: Res<RenderDevice>,
    mut uniform_buffer: ResMut<UniformBuffer>,
    images: Res<RenderAssets<GpuImage>>,
    time: Res<Time>,
    mut last_texture_id: Local<Option<TextureViewId>>,
) {
    let texture = &images
        .get(&noise_config.0)
        .unwrap_or(&fallback_img.d2)
        .texture_view;
    *last_texture_id = Some(texture.id());

    let shader_config =
        ShaderRegionConfiguration::new(region_config.as_ref(), time.elapsed_seconds_wrapped());
    let config_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("region config buffer"),
        contents: bytemuck::bytes_of(&shader_config),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let layout = pipeline.region_layout.clone();
    let bind_group = render_device.create_bind_group(
        "grass uniform bind group ",
        &layout,
        &BindGroupEntries::sequential((
            BindingResource::Buffer(BufferBinding {
                buffer: &config_buffer,
                offset: 0,
                size: None,
            }),
            BindingResource::TextureView(texture),
        )),
    );
    uniform_buffer.set(bind_group);
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderRegionConfiguration {
    /// The time since startup in seconds.
    /// Wraps to 0 after 1 hour
    time: f32,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: f32,
    /// Direction of the wind
    wind: Vec2,
}

impl ShaderRegionConfiguration {
    pub fn new(config: &GrassConfiguration, time: f32) -> ShaderRegionConfiguration {
        Self {
            wind: config.wind,
            _wasm_padding: 0.,
            time,
        }
    }
}
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderAabb {
    vect: Vec3,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: u32,
}

impl From<Vec3> for ShaderAabb {
    fn from(vect: Vec3) -> Self {
        Self {
            vect,
            _wasm_padding: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderColorUniform {
    main_color: Vec4,
    bottom_color: Vec4,
}
impl From<&GrassColor> for ShaderColorUniform {
    fn from(config: &GrassColor) -> Self {
        Self {
            main_color: config.main_color.to_srgba().to_vec4(),
            bottom_color: config.bottom_color.to_srgba().to_vec4(),
        }
    }
}
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderHeightUniform {
    height: f32,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: Vec3,
}

impl From<f32> for ShaderHeightUniform {
    fn from(height: f32) -> Self {
        Self {
            height,
            _wasm_padding: Vec3::ZERO,
        }
    }
}
