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

use crate::grass_spawner::GrassSpawnerFlags;

use super::cache::GrassCache;
pub(crate) struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<GrassCache>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<Handle<Mesh>>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        mesh_handle: &'w Handle<Mesh>,
        (meshes, cache): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };
        let entity = item.entity();
        if !cache.contains_key(&entity) {
            return RenderCommandResult::Failure;
        }
        let chunk = &cache.into_inner()[&entity];
        // set uniforms
        pass.set_bind_group(2, chunk.uniform_bindgroup.as_ref().unwrap(), &[]);
        if chunk.flags.contains(GrassSpawnerFlags::HEIGHT_MAP) {
            pass.set_bind_group(3, chunk.height_map.as_ref().unwrap(), &[]);
        } else {
            pass.set_bind_group(3, chunk.explicit_y_buffer.as_ref().unwrap(), &[]);
        }
        if !chunk.flags.contains(GrassSpawnerFlags::DENSITY_MAP) {
            pass.set_bind_group(4, chunk.explicit_xz_buffer.as_ref().unwrap(), &[]);
            pass.set_bind_group(5, chunk.height_buffer.as_ref().unwrap(), &[]);
        }
        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        let grass_blade_count = chunk.instance_count as u32;
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
