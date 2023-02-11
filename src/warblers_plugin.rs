use bevy::{
    core_pipeline::core_3d::Transparent3d,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::ExtractComponentPlugin,
        extract_resource::ExtractResourcePlugin,
        mesh::Indices,
        render_phase::AddRenderCommand,
        render_resource::{PrimitiveTopology, SpecializedMeshPipelines},
        RenderApp, RenderStage,
    },
};

use crate::{
    
    render::{self, grass_pipeline::GrassPipeline},
    Grass, RegionConfiguration, file_loader::{GrassFields, GrassFieldsAssetLoader},
};

pub(crate) const GRASS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2263343952151597127);
pub(crate) const GRASS_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 9357128457583957921);
pub struct WarblersPlugin;
impl Plugin for WarblersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        {
            let world = app.world.cell();
            // load default grass mesh
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            let mut grass_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            grass_mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [0., 0., 0.],
                    [0.5, 0., 0.],
                    [0.25, 0., 0.4],
                    [0.25, 1., 0.15],
                ],
            );
            grass_mesh.set_indices(Some(Indices::U32(vec![1, 0, 3, 2, 1, 3, 0, 2, 3])));

            meshes.set_untracked(GRASS_MESH_HANDLE, grass_mesh);
            // load shader
            let mut shaders = world.resource_mut::<Assets<Shader>>();
            let grass_shader = Shader::from_wgsl(include_str!("render/grass_shader.wgsl"));
            shaders.set_untracked(GRASS_SHADER_HANDLE, grass_shader);
        }
        app.init_resource::<RegionConfiguration>()
            .register_type::<RegionConfiguration>()
            .add_asset::<GrassFields>()
            .init_asset_loader::<GrassFieldsAssetLoader>();
        app.add_plugin(ExtractComponentPlugin::<Grass>::default());
        app.add_plugin(ExtractResourcePlugin::<RegionConfiguration>::default());

        app.sub_app_mut(RenderApp)
            .init_resource::<GrassPipeline>()
            .init_resource::<SpecializedMeshPipelines<GrassPipeline>>()
            .add_render_command::<Transparent3d, render::GrassDrawCall>()
            .add_system_to_stage(RenderStage::Prepare, render::prepare_instance_buffers)
            .add_system_to_stage(RenderStage::Queue, render::queue_grass_buffers);
    }
}
