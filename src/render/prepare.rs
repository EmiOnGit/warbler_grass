use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroU64;
use std::ops::Mul;

use super::cache::UniformBuffer;
use super::grass_pipeline::GrassPipeline;
use crate::bundle::WarblerHeight;
use crate::map::YMap;
use crate::prelude::GrassColor;
use crate::{GrassConfiguration, GrassNoiseTexture};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding,
    BufferInitDescriptor, BufferUsages, TextureViewId,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
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
#[derive(Component)]
pub(crate) struct UniformHeightFlag;

pub(crate) fn prepare_height_buffer(
    mut commands: Commands,
    pipeline: Res<GrassPipeline>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<Image>>,

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
                let bind_group_descriptor = BindGroupDescriptor {
                    label: Some("grass blade height bind group"),
                    layout: &layout,
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer: &buffer,
                            offset: 0,
                            size: NonZeroU64::new(mem::size_of::<ShaderHeightUniform>() as u64),
                        }),
                    }],
                };
                let bind_group = render_device.create_bind_group(&bind_group_descriptor);
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

                let bind_group_descriptor = BindGroupDescriptor {
                    label: Some("grass height map bind group"),
                    layout: &layout,
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(tex),
                    }],
                };

                let bind_group = render_device.create_bind_group(&bind_group_descriptor);
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
        let bind_group_descriptor = BindGroupDescriptor {
            label: Some("grass color bind group"),
            layout: &layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: NonZeroU64::new(mem::size_of::<ShaderColorUniform>() as u64),
                }),
            }],
        };
        let bind_group = render_device.create_bind_group(&bind_group_descriptor);
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
    images: Res<RenderAssets<Image>>,
    inserted_grass: Query<(Entity, &YMap, &Aabb)>,
) {
    let layout = pipeline.y_map_layout.clone();

    for (entity, y_map, aabb) in inserted_grass.iter() {
        let y_map_texture = if let Some(tex) = images.get(&y_map.y_map) {
            &tex.texture_view
        } else {
            &fallback_img.d2.texture_view
        };

        let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("aabb buffer"),
            contents: bytemuck::bytes_of(&ShaderAabb::from(Vec3::from(aabb.half_extents.mul(2.)))),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_descriptor = BindGroupDescriptor {
            label: Some("grass y-map bind group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(y_map_texture),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &aabb_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        };

        let bind_group = render_device.create_bind_group(&bind_group_descriptor);
        commands
            .entity(entity)
            .insert(BindGroupBuffer::<YMap>::new(bind_group));
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
    images: Res<RenderAssets<Image>>,
    mut last_texture_id: Local<Option<TextureViewId>>,
) {
    let texture = &images
        .get(&noise_config.0)
        .unwrap_or(&fallback_img.d2)
        .texture_view;
    if !region_config.is_changed() && Some(texture.id()) == *last_texture_id {
        return;
    }
    *last_texture_id = Some(texture.id());

    let shader_config = ShaderRegionConfiguration::from(region_config.as_ref());
    let config_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("region config buffer"),
        contents: bytemuck::bytes_of(&shader_config),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let layout = pipeline.region_layout.clone();
    let bind_group_descriptor = BindGroupDescriptor {
        label: Some("grass uniform bind group"),
        layout: &layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &config_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(texture),
            },
        ],
    };
    let bind_group = render_device.create_bind_group(&bind_group_descriptor);
    uniform_buffer.set(bind_group);
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderRegionConfiguration {
    wind: Vec2,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: Vec2,
}

impl From<&GrassConfiguration> for ShaderRegionConfiguration {
    fn from(config: &GrassConfiguration) -> Self {
        Self {
            wind: config.wind,
            _wasm_padding: Vec2::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ShaderAabb {
    vect: Vec3,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: f32,
}

impl From<Vec3> for ShaderAabb {
    fn from(vect: Vec3) -> Self {
        Self {
            vect,
            _wasm_padding: 0.,
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
            main_color: config.main_color.into(),
            bottom_color: config.bottom_color.into(),
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
