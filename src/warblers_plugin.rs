use bevy::{
    core_pipeline::core_3d::Opaque3d,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_resource::ExtractResourcePlugin,
        mesh::Indices,
        render_phase::AddRenderCommand,
        render_resource::{PrimitiveTopology, SpecializedMeshPipelines},
        texture::FallbackImage,
        RenderApp, RenderStage,
    }, asset::load_internal_asset,
};

use crate::{
    file_loader::{GrassFields, GrassFieldsAssetLoader},
    render::{self, grass_pipeline::GrassPipeline, cache::GrassCache}, RegionConfiguration, prelude::add_aabb_box_to_grass,
};

pub(crate) const GRASS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2263343952151597127);
pub(crate) const GRASS_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 9357128457583957921);

pub struct WarblersPlugin;
impl Plugin for WarblersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Load grass shader into cache
        load_internal_asset!(
            app,
            GRASS_SHADER_HANDLE,
            "render/assets/grass_shader.wgsl",
            Shader::from_wgsl
        );

        // Load default grass blade mesh
        let mut meshes = app.world.resource_mut::<Assets<Mesh>>();
        meshes.set_untracked(GRASS_MESH_HANDLE, default_grass_mesh());
        // Add systems
        app.add_system(add_aabb_box_to_grass);
        // Init resources
        app.init_resource::<RegionConfiguration>()
            .register_type::<RegionConfiguration>()
            .add_asset::<GrassFields>()
            .init_asset_loader::<GrassFieldsAssetLoader>();
        // Add extraction
        app
            .add_plugin(ExtractResourcePlugin::<RegionConfiguration>::default());
        // Init render app
        app.sub_app_mut(RenderApp)
            .add_render_command::<Opaque3d, render::GrassDrawCall>()
            .init_resource::<FallbackImage>()
            .init_resource::<GrassPipeline>()
            .init_resource::<GrassCache>()
            .init_resource::<SpecializedMeshPipelines<GrassPipeline>>()
            .add_system_to_stage(RenderStage::Extract, render::extract::extract_grass)
            .add_system_to_stage(RenderStage::Prepare, render::prepare_instance_buffers)
            .add_system_to_stage(RenderStage::Queue, render::queue_grass_buffers);
    }
}

/// simply the default look of the grass, as shown in the examples
fn default_grass_mesh() -> Mesh {
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
        grass_mesh
}