use super::grass_pipeline::GrassPipeline;
use crate::render::cache::GrassCache;
use crate::RegionConfiguration;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages, ShaderType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
use bytemuck::{Pod, Zeroable};

pub(crate) fn prepare_instance_buffer(
    mut cache: ResMut<GrassCache>,
    render_device: Res<RenderDevice>,
) {
    if !cache.is_changed() {
        return;
    }
    for instance_data in cache.values_mut() {
        let entity_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Instance entity buffer"),
            contents: bytemuck::cast_slice(&instance_data.grass.instances.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        instance_data.grass_buffer = Some(entity_buffer);
    }
}

pub(crate) fn prepare_uniform_buffers(
    pipeline: Res<GrassPipeline>,
    mut cache: ResMut<GrassCache>,
    region_config: Res<RegionConfiguration>,
    fallback_img: Res<FallbackImage>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
) {
    if !region_config.is_changed() {
        return;
    }
    let shader_config = ShaderRegionConfiguration {
        main_color: region_config.main_color.into(),
        bottom_color: region_config.bottom_color.into(),
        wind: region_config.wind,
        _wasm_padding: Vec2::ZERO,
    };
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
                resource: BindingResource::TextureView({
                    if let Some(img) = images.get(&region_config.wind_noise_texture) {
                        &img.texture_view
                    } else {
                        &fallback_img.texture_view
                    }
                }),
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
    pub main_color: Vec4,
    pub bottom_color: Vec4,
    pub wind: Vec2,
    pub _wasm_padding: Vec2,
}
