use bevy::{
    ecs::system::{
        lifetimeless::{Read, SRes},
        SystemParamItem,
    },
    prelude::*,
    render::{
        mesh::GpuBufferInfo,
        render_asset::RenderAssets,
        render_phase::{PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass},
    },
};

// use crate::grass_spawner::GrassSpawnerFlags;

use crate::dithering::{GpuDitheredBuffer, DitheredBuffer};

use super::cache::GrassCache;
pub struct SetUniformBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUniformBindGroup<I> {
    type Param = SRes<GrassCache>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = ();

    fn render<'w>(
        item: &P,
        _view: (),
        _entity: (),
        cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        println!("set uniform");

        let Some(chunk) = cache.into_inner().get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        pass.set_bind_group(I, chunk.uniform_bindgroup.as_ref().unwrap(), &[]);
        println!("f uniform");

        RenderCommandResult::Success
    }
}
pub struct SetYBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetYBindGroup<I> {
    type Param = SRes<GrassCache>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = ();

    fn render<'w>(
        item: &P,
        _view: (),
        _entity: (),
        cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        println!("set y");

        let Some(chunk) = cache.into_inner().get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        // if chunk.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            pass.set_bind_group(I, chunk.height_map.as_ref().unwrap(), &[]);
        // } else {
            // pass.set_bind_group(I, chunk.explicit_y_buffer.as_ref().unwrap(), &[]);
        // }
        println!("f y");

        RenderCommandResult::Success
    }
}
pub struct SetHeightBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetHeightBindGroup<I> {
    type Param = SRes<GrassCache>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = ();

    fn render<'w>(
        item: &P,
        _view: (),
        _entity: (),
        cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        println!("set height");
        let Some(chunk) = cache.into_inner().get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        pass.set_bind_group(I, chunk.height_buffer.as_ref().unwrap(), &[]);
        println!("finished height");

        RenderCommandResult::Success
    }
}
pub(crate) struct SetVertexBuffer;

impl<P: PhaseItem> RenderCommand<P> for SetVertexBuffer {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<GrassCache>, SRes<RenderAssets<DitheredBuffer>>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<Handle<Mesh>>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        (mesh_handle): &'w Handle<bevy::prelude::Mesh>,
        (meshes, cache, dither): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };
        let Some(chunk) = cache.into_inner().get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        let dither_handle = chunk.dither_handle.as_ref().unwrap();
        let gpu_dither = match dither.into_inner().get(dither_handle) {
            Some(gpu_dither) => gpu_dither,
            None => return RenderCommandResult::Failure,
        };
        let grass_blade_count = gpu_dither.instances as u32;

        if grass_blade_count == 0 {
            return RenderCommandResult::Failure;
        }
        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, gpu_dither.buffer.slice(..));
        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..grass_blade_count);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..grass_blade_count);
            }
        }
        RenderCommandResult::Success
    }
}
