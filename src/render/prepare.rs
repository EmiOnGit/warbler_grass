use std::ops::Mul;

use super::extract::EntityStore;
use super::grass_pipeline::GrassPipeline;
use crate::grass_spawner::{GrassSpawner, GrassSpawnerFlags, HeightRepresentation};
use crate::render::cache::GrassCache;
use crate::RegionConfiguration;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages, ShaderType, TextureViewId,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
use bytemuck::{Pod, Zeroable};

pub(crate) fn prepare_instance_buffer(
    mut cache: ResMut<GrassCache>,
    render_device: Res<RenderDevice>,
    inserted_grass: Query<(&GrassSpawner, &EntityStore)>,
) {
    for (spawner, EntityStore(id)) in inserted_grass.iter() {
        if !spawner.flags.contains(GrassSpawnerFlags::Y_DEFINED) {
            panic!("Cannot spawn grass without the y-positions defined");
        }
        if !spawner.flags.contains(GrassSpawnerFlags::XZ_DEFINED) {
            panic!("Cannot spawn grass without the xz-positions defined");
        }
        let heights = match &spawner.heights {
            HeightRepresentation::Uniform(height) => vec![*height; spawner.positions_xz.len()],
            HeightRepresentation::PerBlade(heights) => heights.clone(),
        };
        let instance_slice: Vec<Vec4> = if spawner.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            spawner
                .positions_xz
                .iter()
                .zip(heights)
                .map(|(xz, height)| Vec4::new(xz.x, 0.0, xz.y, height))
                .collect()
        } else {
            spawner
                .positions_xz
                .iter()
                .zip(spawner.positions_y.iter())
                .zip(heights)
                .map(|((xz, y), height)| Vec4::new(xz.x, *y, xz.y, height))
                .collect()
        };
        if let Some(chunk) = cache.get_mut(&id) {
            chunk.instances = Some(instance_slice);
            let inst = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("Instance entity buffer"),
                contents: bytemuck::cast_slice(chunk.instances.as_ref().unwrap().as_slice()),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });
            chunk.instance_buffer = Some(inst);
            chunk.flags = spawner.flags;
        } else {
            warn!(
                "Tried to prepare a entity buffer for a grass chunk which wasn't registered before"
            );
        }
    }
}
pub(crate) fn prepare_height_map_buffer(
    mut cache: ResMut<GrassCache>,
    render_device: Res<RenderDevice>,
    pipeline: Res<GrassPipeline>,
    fallback_img: Res<FallbackImage>,
    images: Res<RenderAssets<Image>>,
    inserted_grass: Query<(&GrassSpawner, &EntityStore, &Aabb)>,
    mut local_height_map_buffer: Local<Vec<(EntityStore, Handle<Image>, Aabb)>>,
) {
    let mut to_remove = Vec::new();

    for (EntityStore(e), handle, aabb) in local_height_map_buffer.iter() {
        if let Some(tex) = images.get(&handle) {
            to_remove.push(*e);
            let height_map_texture = &tex.texture_view;
            let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("aabb buffer"),
                contents: bytemuck::bytes_of(&aabb.half_extents.mul(2.).as_dvec3().as_vec3()),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });
            let layout = pipeline.height_map_layout.clone();
            let bind_group_descriptor = BindGroupDescriptor {
                label: Some("grass height map bind group"),
                layout: &layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(height_map_texture),
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
            if let Some(chunk) = cache.get_mut(&e) {
                chunk.height_map = Some(bind_group);
            } else {
                warn!("Tried to prepare a buffer for a grass chunk which wasn't registered before");
            }
        }
    }
    local_height_map_buffer.retain(|map| !to_remove.contains(&map.0 .0));
    for (spawner, entity_store, aabb) in inserted_grass.iter() {
        let id = entity_store.0;
        if spawner.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            let handle = &spawner.height_map.as_ref().unwrap().height_map;
            if images.get(&handle).is_none() {
                local_height_map_buffer.push((entity_store.clone(), handle.clone(), aabb.clone()));
            }
        }
        let (height_map_texture, aabb_buffer) =
            if !spawner.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
                let height_map_texture = &fallback_img.texture_view;
                let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                    label: Some("aabb buffer"),
                    contents: &[0],
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                });
                (height_map_texture, aabb_buffer)
            } else {
                let handle = spawner.height_map.as_ref().unwrap().height_map.clone();
                let height_map_texture = if let Some(tex) = images.get(&handle) {
                    &tex.texture_view
                } else {
                    &fallback_img.texture_view
                };

                let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                    label: Some("aabb buffer"),
                    contents: bytemuck::bytes_of(&aabb.half_extents.mul(2.).as_dvec3().as_vec3()),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                });
                (height_map_texture, aabb_buffer)
            };
        let layout = pipeline.height_map_layout.clone();

        let bind_group_descriptor = BindGroupDescriptor {
            label: Some("grass height map bind group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(height_map_texture),
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
        if let Some(chunk) = cache.get_mut(&id) {
            chunk.height_map = Some(bind_group);
        } else {
            warn!("Tried to prepare a buffer for a grass chunk which wasn't registered before");
        }
    }
}
pub(crate) fn prepare_uniform_buffers(
    pipeline: Res<GrassPipeline>,
    mut cache: ResMut<GrassCache>,
    region_config: Res<RegionConfiguration>,
    fallback_img: Res<FallbackImage>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    mut last_texture_id: Local<Option<TextureViewId>>,
) {
    let texture = &images
        .get(&region_config.wind_noise_texture)
        .unwrap_or(&fallback_img)
        .texture_view;
    if !region_config.is_changed() && Some(texture.id()) == *last_texture_id && !cache.is_changed()
    {
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

    for instance_data in cache.values_mut() {
        instance_data.uniform_bindgroup = Some(bind_group.clone());
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable, ShaderType)]
#[repr(C)]
struct ShaderRegionConfiguration {
    main_color: Vec4,
    bottom_color: Vec4,
    wind: Vec2,
    /// Wasm requires shader uniforms to be aligned to 16 bytes
    _wasm_padding: Vec2,
}

impl From<&RegionConfiguration> for ShaderRegionConfiguration {
    fn from(config: &RegionConfiguration) -> Self {
        Self {
            main_color: config.main_color.into(),
            bottom_color: config.bottom_color.into(),
            wind: config.wind,
            _wasm_padding: Vec2::ZERO,
        }
    }
}
