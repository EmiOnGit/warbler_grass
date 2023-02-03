use bevy::{
    core_pipeline::core_3d::Transparent3d,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::ExtractComponentPlugin, render_phase::AddRenderCommand,
        render_resource::SpecializedMeshPipelines, RenderApp, RenderStage, extract_resource::ExtractResourcePlugin,
    },
};

use crate::{
    render::{self, grass_pipeline::GrassPipeline},
    GrassData, RegionConfig,
};

pub(crate) const GRASS_RENDER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2263343952151597127);

pub struct WarblersPlugin;
impl Plugin for WarblersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        let grass_shader = Shader::from_wgsl(include_str!("render/grass_shader.wgsl"));
        shaders.set_untracked(GRASS_RENDER_HANDLE, grass_shader);
        app.init_resource::<RegionConfig>()
            .register_type::<RegionConfig>();
        app.add_plugin(ExtractComponentPlugin::<GrassData>::default());
        app.add_plugin(ExtractResourcePlugin::<RegionConfig>::default());

        app.sub_app_mut(RenderApp)
            .init_resource::<GrassPipeline>()
            .init_resource::<SpecializedMeshPipelines<GrassPipeline>>()
            .add_render_command::<Transparent3d, render::GrassDrawCall>()
            .add_system_to_stage(RenderStage::Prepare, render::prepare_instance_buffers)
            .add_system_to_stage(RenderStage::Queue, render::queue_grass_buffers);
    }
}
