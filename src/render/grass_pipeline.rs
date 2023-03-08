use bevy::{
    pbr::{MeshPipeline, MeshPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
            BufferBindingType, RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipeline,
            SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension, VertexAttribute,
            VertexBufferLayout, VertexFormat, VertexStepMode,
        },
        renderer::RenderDevice,
    },
};

use crate::{warblers_plugin::GRASS_SHADER_HANDLE};
#[derive(Resource)]
pub struct GrassPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub region_layout: BindGroupLayout,
    pub height_map_layout: BindGroupLayout,
    pub density_map_layout: BindGroupLayout,
    pub explicit_y_layout: BindGroupLayout,
    pub explicit_height_layout: BindGroupLayout,
    pub uniform_height_layout: BindGroupLayout,
    pub explicit_xz_layout: BindGroupLayout,
}

impl FromWorld for GrassPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let region_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("warblersneeds configuration layout"),
            entries: &[
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
        });
        let height_map_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warblersneeds configuration layout"),
                entries: &[
                    // height_map
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
            });
        let density_map_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warblersneeds density map layout"),
                entries: &[
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
            });
        let explicit_y_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warbler_grass explicit y layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let explicit_height_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warbler_grass explicit height layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let explicit_xz_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warbler_grass explicit height layout"),
                entries: &[
                    // heights
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
            });
        let uniform_height_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("warblersneeds configuration layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let shader = GRASS_SHADER_HANDLE.typed::<Shader>();
        let mesh_pipeline = world.resource::<MeshPipeline>();
        GrassPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            region_layout,
            explicit_height_layout,
            uniform_height_layout,
            density_map_layout,
            explicit_xz_layout,
            explicit_y_layout,
            height_map_layout,
        }
    }
}
impl SpecializedMeshPipeline for GrassPipeline {
    type Key = GrassRenderKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key.mesh_key, layout)?;
        descriptor.label = Some("Grass Render Pipeline".into());
        descriptor.layout.push(self.region_layout.clone());
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<Vec2>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 3, // shader locations 0-2 may be taken up by Position, Normal and UV attributes
            }],
        });
        let vertex = &mut descriptor.vertex;

        vertex.shader = self.shader.clone();
        // if key.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            vertex.shader_defs.push("HEIGHT_MAP".into());
            descriptor.layout.push(self.height_map_layout.clone());
        // } else {
            // descriptor.layout.push(self.explicit_y_layout.clone());
        // }
        // if key.flags.contains(GrassSpawnerFlags::DENSITY_MAP) {
            vertex.shader_defs.push("UNIFORM_HEIGHT".into());
            descriptor.layout.push(self.uniform_height_layout.clone());
        // } else {
            // descriptor.layout.push(self.explicit_height_layout.clone());
        // }

        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct GrassRenderKey {
    pub mesh_key: MeshPipelineKey,
    // flags: GrassSpawnerFlags,
}

impl From<MeshPipelineKey> for GrassRenderKey {
    fn from(mesh_key: MeshPipelineKey) -> Self {
        Self {
            mesh_key,
            // flags: GrassSpawnerFlags::NONE,
        }
    }
}
// impl GrassRenderKey {
//     pub fn with_flags(mut self, flags: GrassSpawnerFlags) -> Self {
//         self.flags = flags;
//         self
//     }
// }
