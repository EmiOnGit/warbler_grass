use bevy::{
    pbr::{MeshPipeline, MeshPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType,
            RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipeline,
            SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension, VertexAttribute,
            VertexBufferLayout, VertexFormat, VertexStepMode,
        },
        renderer::RenderDevice,
    },
};

use crate::warblers_plugin::GRASS_SHADER_HANDLE;
#[derive(Resource)]
pub struct GrassPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub region_layout: BindGroupLayout,
    pub y_map_layout: BindGroupLayout,
    pub normal_map_layout: BindGroupLayout,
    pub density_map_layout: BindGroupLayout,
    pub heights_texture_layout: BindGroupLayout,
    pub uniform_height_layout: BindGroupLayout,
    pub color_layout: BindGroupLayout,
    pub instance_index_bind_group_layout: BindGroupLayout,
}

impl FromWorld for GrassPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let region_layout = render_device.create_bind_group_layout(
            Some("warbler_grass configuration layout"),
            &[
                // config
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Wind noise Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        );
        let y_map_layout = render_device.create_bind_group_layout(
            Some("warbler_grass y map layout"),
            &[
                // y_texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // aabb box
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );
        let normal_map_layout = render_device.create_bind_group_layout(
            Some("warbler_grass normal map layout"),
            &[
                // normal_texture_view
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        );
        let density_map_layout = render_device.create_bind_group_layout(
            Some("warbler_grass density map layout"),
            &[
                // density map
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        );

        let heights_texture_layout = render_device.create_bind_group_layout(
            Some("warbler_grass height texture layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        );
        let uniform_height_layout = render_device.create_bind_group_layout(
            Some("warbler_grasss configuration layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        );
        let color_layout = render_device.create_bind_group_layout(
            Some("warbler_grass color layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        );

        let instance_index_bind_group_layout = render_device.create_bind_group_layout(
            Some("instance index bind group layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        );

        let mesh_pipeline = world.resource::<MeshPipeline>();
        GrassPipeline {
            shader: GRASS_SHADER_HANDLE,
            mesh_pipeline: mesh_pipeline.clone(),
            region_layout,
            uniform_height_layout,
            heights_texture_layout,
            density_map_layout,
            y_map_layout,
            normal_map_layout,
            color_layout,
            instance_index_bind_group_layout,
        }
    }
}
impl SpecializedMeshPipeline for GrassPipeline {
    type Key = GrassRenderKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key.mesh_key, layout)?;
        let vertex = &mut descriptor.vertex;
        descriptor.label = Some("Grass Render Pipeline".into());
        // set shaders
        vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        // set shader defs
        vertex.shader_defs.push("MESH_BINDGROUP_1".into());
        descriptor
            .fragment
            .as_mut()
            .unwrap()
            .shader_defs
            .push("MESH_BINDGROUP_1".into());
        if !key.uniform_height {
            vertex.shader_defs.push("HEIGHT_TEXTURE".into());
        }
        // set buffers
        vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<Vec2>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 3, // shader locations 0-2 may be taken up by Position, Normal and UV attributes
            }],
        });

        // set layouts
        // layouts that are added to the descriptor.
        // Order of elements has to correspond with the groups
        let layouts = [
            if key.uniform_height {
                &self.uniform_height_layout
            } else {
                &self.heights_texture_layout
            },
            &self.color_layout,
            &self.y_map_layout,
            &self.region_layout,
            &self.normal_map_layout,
        ];
        for layout in layouts {
            descriptor.layout.push(layout.clone());
        }
        Ok(descriptor)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct GrassRenderKey {
    pub mesh_key: MeshPipelineKey,
    pub uniform_height: bool,
}

impl From<MeshPipelineKey> for GrassRenderKey {
    fn from(mesh_key: MeshPipelineKey) -> Self {
        Self {
            mesh_key,
            uniform_height: false,
        }
    }
}
