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

use crate::{grass_spawner::GrassSpawnerFlags, warblers_plugin::GRASS_SHADER_HANDLE};
#[derive(Resource)]
pub struct GrassPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub region_layout: BindGroupLayout,
    pub height_map_layout: BindGroupLayout,
    pub flags: u32,
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
        let shader = GRASS_SHADER_HANDLE.typed::<Shader>();
        let mesh_pipeline = world.resource::<MeshPipeline>();
        GrassPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            region_layout,
            height_map_layout,
            flags: 0,
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
        let vertex = &mut descriptor.vertex;
        vertex.shader = self.shader.clone();
        if key.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            vertex.shader_defs.push("HEIGHT_MAP".into());
        }
        descriptor.layout.push(self.region_layout.clone());
        descriptor.layout.push(self.height_map_layout.clone());
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: VertexFormat::Float32x4.size(),
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // position of the mesh as instance
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 1,
                },
                // height scale
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: VertexFormat::Float32x3.size(),
                    shader_location: 2,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct GrassRenderKey {
    pub mesh_key: MeshPipelineKey,
    flags: GrassSpawnerFlags,
}

impl From<MeshPipelineKey> for GrassRenderKey {
    fn from(mesh_key: MeshPipelineKey) -> Self {
        Self {
            mesh_key,
            flags: GrassSpawnerFlags::NONE,
        }
    }
}
impl GrassRenderKey {
    pub fn with_flags(mut self, flags: GrassSpawnerFlags) -> Self {
        self.flags = flags;
        self
    }
}
