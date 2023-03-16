use bevy::{
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
    utils::{HashMap, HashSet},
};

use crate::dithering::DitheredBuffer;

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub(crate) struct GrassCache {
    pub data: HashMap<Entity, CachedGrassChunk>,
}

#[derive(Debug, Default)]
pub(crate) struct CachedGrassChunk {
    pub uniform_bindgroup: Option<BindGroup>,
    pub explicit_xz_buffer: Option<Buffer>,
    pub explicit_count: u32,
    pub dither_handle: Option<Handle<DitheredBuffer>>,
}

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct EntityCache {
    pub entities: HashSet<Entity>,
}
