use bevy::{
    ecs::system::{
        lifetimeless::{Read, SRes},
        SystemParamItem,
    },
    pbr::RenderMeshInstances,
    prelude::*,
    render::{
        mesh::GpuBufferInfo,
        render_asset::RenderAssets,
        render_phase::{PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass},
    },
};

use crate::{
    dithering::DitheredBuffer,
    map::YMap,
    prelude::{GrassColor, NormalMap, WarblerHeight},
};

use super::{
    cache::UniformBuffer,
    prepare::{BindGroupBuffer, IndexBindgroup},
};
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
        RenderCommandResult::Success
    }
}
pub(crate) struct SetNormalBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetNormalBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Option<Read<BindGroupBuffer<NormalMap>>>;

    fn render<'w>(
        _item: &P,
        _view: (),
        bind_group: Option<&'w BindGroupBuffer<NormalMap>>,
        _cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(bind_group) = bind_group else {
            return RenderCommandResult::Failure;
        };
        pass.set_bind_group(I, &bind_group.bind_group, &[]);
        RenderCommandResult::Success
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
pub(crate) struct SetInstanceIndexBindGroup<const N: usize>;

impl<P: PhaseItem, const N: usize> RenderCommand<P> for SetInstanceIndexBindGroup<N> {
    type Param = ();
    type ViewWorldQuery = ();

    type ItemWorldQuery = Read<IndexBindgroup>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        index_bindgroup: &'w IndexBindgroup,
        _: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(N, &index_bindgroup.bind_group, &[]);
        RenderCommandResult::Success
    }
}
pub(crate) struct SetVertexBuffer;

impl<P: PhaseItem> RenderCommand<P> for SetVertexBuffer {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SRes<RenderMeshInstances>,
        SRes<RenderAssets<DitheredBuffer>>,
    );
    type ViewWorldQuery = ();
    type ItemWorldQuery = Option<Read<Handle<DitheredBuffer>>>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        dither_handle: Option<&'w Handle<DitheredBuffer>>,
        (meshes, render_mesh_instances, dither): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Failure;
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
