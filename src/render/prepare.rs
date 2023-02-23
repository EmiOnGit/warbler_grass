use crate::RegionConfiguration;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;

use crate::render::cache::GrassCache;

use super::grass_pipeline::GrassPipeline;

pub(crate) fn prepare_uniform_buffers(mut cache: ResMut<GrassCache>, render_device: Res<RenderDevice>) {
    if !cache.is_changed() {
        return;
    }
    for instance_data in cache.values_mut() {
        let entity_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Instance entity buffer"),
            contents: bytemuck::cast_slice(&instance_data.grass.instances.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        instance_data.buffer = Some(entity_buffer);
    }
}

pub(crate) fn prepare_instance_buffer(
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
    for instance_data in cache.values_mut() {
        let region_color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("region color buffer"),
            contents: bytemuck::cast_slice(&region_config.main_color.as_rgba_f32()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let region_bottom_color_buffer =
            render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("region bottom color buffer"),
                contents: bytemuck::cast_slice(&region_config.bottom_color.as_rgba_f32()),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });
        let region_wind_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("region wind buffer"),
            contents: bytemuck::cast_slice(&region_config.wind.to_array()),
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
                        buffer: &region_color_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &region_bottom_color_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &region_wind_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 3,
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
        instance_data.uniform_bindgroup = Some(bind_group);
    }
}
