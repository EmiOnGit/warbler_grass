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

use crate::{
    dithering::DitheredBuffer,
    y_map::YMap,
    prelude::{GrassColor, WarblerHeight},
};

use super::{cache::UniformBuffer, prepare::BindGroupBuffer};
pub(crate) struct SetUniformBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUniformBindGroup<I> {
    type Param = SRes<UniformBuffer>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = ();

    fn render<'w>(
        _item: &P,
        _view: (),
        _entity: (),
        cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, cache.into_inner().ref_unwrap(), &[]);

        RenderCommandResult::Success
    }
}
pub(crate) struct SetYBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetYBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Option<Read<BindGroupBuffer<YMap>>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        bind_group: Option<&'w BindGroupBuffer<YMap>>,
        _cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(bind_group) = bind_group else {
            return RenderCommandResult::Failure;
        };
        pass.set_bind_group(I, &bind_group.bind_group, &[]);
        return RenderCommandResult::Success;
    }
}
pub(crate) struct SetColorBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetColorBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<BindGroupBuffer<GrassColor>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        color: &'w BindGroupBuffer<GrassColor>,
        _param: (),
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &color.bind_group, &[]);

        RenderCommandResult::Success
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
    type Param = (SRes<RenderAssets<Mesh>>, SRes<RenderAssets<DitheredBuffer>>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = (Read<Handle<Mesh>>, Option<Read<Handle<DitheredBuffer>>>);

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        (mesh_handle, dither_handle): (
            &'w Handle<bevy::prelude::Mesh>,
            Option<&'w Handle<DitheredBuffer>>,
        ),
        (meshes, dither): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        let blade_count;

        if let Some(dither_handle) = dither_handle {
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
            return RenderCommandResult::Failure;
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
            GpuBufferInfo::NonIndexed => {
                pass.draw(0..gpu_mesh.vertex_count, 0..blade_count);
            }
        }
        RenderCommandResult::Success
    }
}
