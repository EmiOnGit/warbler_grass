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

use crate::{dithering::DitheredBuffer, height_map::HeightMap, prelude::WarblerHeight};

use super::{cache::GrassCache, prepare::BindGroupBuffer};
pub(crate) struct SetUniformBindGroup<const I: usize>;

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
        let Some(chunk) = cache.into_inner().get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        pass.set_bind_group(I, chunk.uniform_bindgroup.as_ref().unwrap(), &[]);

        RenderCommandResult::Success
    }
}
pub(crate) struct SetYBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetYBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Option<Read<BindGroupBuffer<HeightMap>>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        bind_group: Option<&'w BindGroupBuffer<HeightMap>>,
        _cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // let Some(chunk) = cache.into_inner().get(&item.entity()) else {
        //     return RenderCommandResult::Failure;
        // };
        // if let Some(height_map) = chunk.height_map.as_ref() {
        //     pass.set_bind_group(I, height_map, &[]);
        //     return RenderCommandResult::Success;
        // }
        // if let Some(y_buffer) = chunk.explicit_y_buffer.as_ref() {
        //     pass.set_bind_group(I, y_buffer, &[]);
        //     return RenderCommandResult::Success;
        // }
        // RenderCommandResult::Failure
        let Some(bind_group) = bind_group else {
            println!("couldn't find bind group buffer height map");
            return RenderCommandResult::Failure;

        };
        pass.set_bind_group(I, &bind_group.bind_group, &[]);
        return RenderCommandResult::Success;
    }
}
pub(crate) struct SetHeightBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetHeightBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<BindGroupBuffer<WarblerHeight>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        height: &'w BindGroupBuffer<WarblerHeight>,
        _param: (),
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &height.bind_group, &[]);

        RenderCommandResult::Success
    }
}
pub(crate) struct SetVertexBuffer;

impl<P: PhaseItem> RenderCommand<P> for SetVertexBuffer {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SRes<GrassCache>,
        SRes<RenderAssets<DitheredBuffer>>,
    );
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<Handle<Mesh>>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        mesh_handle: &'w Handle<bevy::prelude::Mesh>,
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
        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        let blade_count;

        if let Some(dither_handle) = chunk.dither_handle.as_ref() {
            if let Some(gpu_dither) = dither.into_inner().get(dither_handle) {
                blade_count = gpu_dither.instances as u32;
                if blade_count == 0 {
                    return RenderCommandResult::Failure;
                }
                pass.set_vertex_buffer(1, gpu_dither.buffer.slice(..));
            } else {
                return RenderCommandResult::Failure;
            }
        } else {
            blade_count = chunk.explicit_count;
            let Some(xz_buffer) = chunk.explicit_xz_buffer.as_ref() else {
                return RenderCommandResult::Failure;
            };
            pass.set_vertex_buffer(1, xz_buffer.slice(..));
        }

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..blade_count);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..blade_count);
            }
        }
        RenderCommandResult::Success
    }
}
