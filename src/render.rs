use bevy::{
    pbr::{SetMeshBindGroup, SetMeshViewBindGroup},
    render::render_phase::SetItemPipeline,
};

use self::draw::{SetYBindGroup, SetUniformBindGroup, SetHeightBindGroup};

pub(crate) mod cache;
mod draw;
pub(crate) mod extract;
pub(crate) mod grass_pipeline;
pub(crate) mod prepare;
pub(crate) mod queue;

pub(crate) type GrassDrawCall = (
    // Caches the pipeline for next call
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetUniformBindGroup<2>,
    SetYBindGroup<3>,
    SetHeightBindGroup<4>,
    draw::SetVertexBuffer,
);
