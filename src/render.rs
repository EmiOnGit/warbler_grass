use bevy::{
    pbr::{SetMeshBindGroup, SetMeshViewBindGroup},
    render::render_phase::SetItemPipeline,
};

pub(crate) mod cache;
mod draw_mesh;
pub(crate) mod extract;
pub(crate) mod grass_pipeline;
pub(crate) mod prepare;
pub(crate) mod queue;

pub(crate) type GrassDrawCall = (
    // caches pipeline instead of reinit every call
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    draw_mesh::DrawMeshInstanced,
);
