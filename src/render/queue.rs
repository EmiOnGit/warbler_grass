use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::pbr::{MeshPipelineKey, MeshUniform};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{DrawFunctions, RenderPhase};
use bevy::render::render_resource::{PipelineCache, SpecializedMeshPipelines};
use bevy::render::view::ExtractedView;

use super::cache::GrassCache;
use super::grass_pipeline::{GrassPipeline, GrassRenderKey};
use super::GrassDrawCall;

#[allow(clippy::too_many_arguments)]
pub fn queue_grass_buffers(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    grass_pipeline: Res<GrassPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<GrassPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    grass_cacher: Res<GrassCache>,
    meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<(Entity, &MeshUniform, &Handle<Mesh>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<GrassDrawCall>()
        .unwrap();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut opaque_phase) in &mut views {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for (entity, mesh_uniform, mesh_handle) in material_meshes
            .iter()
            .filter(|(e, _, _)| grass_cacher.contains_key(e))
        {
            if let Some(mesh) = meshes.get(mesh_handle) {
                let mesh_key =
                    view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let mut grass_key = GrassRenderKey::from(mesh_key);
                grass_key.is_explicit = grass_cacher[&entity].explicit_xz_buffer.is_some();
                grass_key.uniform_height = grass_cacher[&entity].blade_height_texture.is_none();
                let pipeline = pipelines
                    .specialize(&pipeline_cache, &grass_pipeline, grass_key, &mesh.layout)
                    .unwrap();
                opaque_phase.add(Opaque3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: rangefinder.distance(&mesh_uniform.transform),
                });
            }
        }
    }
}
