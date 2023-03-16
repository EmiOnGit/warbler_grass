use bevy::{
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
    utils::HashMap,
};

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub(crate) struct ExplicitGrassCache {
    pub data: HashMap<Entity, CachedExplicitGrassChunk>,
}

#[derive(Debug, Default)]
pub(crate) struct CachedExplicitGrassChunk {
    pub explicit_xz_buffer: Option<Buffer>,
    pub explicit_count: u32,
}
#[derive(Resource, Default)]
pub(crate) struct UniformBuffer(pub Option<BindGroup>);
impl UniformBuffer {
    pub fn set(&mut self, val: BindGroup) {
        self.0 = Some(val);
    }
    pub fn ref_unwrap(&self) -> &BindGroup {
        self.0.as_ref().unwrap()
    }
}
