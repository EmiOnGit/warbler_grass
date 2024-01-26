use bevy::{
    pbr::{SetMeshBindGroup, SetMeshViewBindGroup},
    render::render_phase::SetItemPipeline,
};

use self::draw::{
    SetColorBindGroup, SetHeightBindGroup, SetNormalBindGroup, SetUniformBindGroup,
    SetVertexBuffer, SetYBindGroup,
};

pub(crate) mod cache;
mod draw;
pub(crate) mod extract;
pub(crate) mod grass_pipeline;
pub(crate) mod prepare;
pub(crate) mod queue;

// The main render call used for the grass render pipeline
pub(crate) type GrassDrawCall = (
    // Caches the pipeline for next call
    SetItemPipeline,
    // Set bind groups from mesh views.
    // In the long run this should be strapped into our own implementation only processing what we actually need
    SetMeshViewBindGroup<0>,
    // Binds the default mesh bind group to the vertex buffer
    SetMeshBindGroup<1>,
    // Binds the height of all the grass blades
    SetHeightBindGroup<2>,
    // Binds the color of all the grass blades
    SetColorBindGroup<3>,
    // Binds the [`GrassConfiguration`](crate::GrassConfiguration)
    SetUniformBindGroup<5>,
    // Bind group for the y position lookup of the blades
    SetYBindGroup<4>,
    // Binds the normals of all the grass blades
    SetNormalBindGroup<6>,
    // Binds the xz position of the grass instances to the vertex buffer
    SetVertexBuffer,
);
