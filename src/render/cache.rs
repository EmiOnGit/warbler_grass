use bevy::{prelude::*, utils::HashMap, render::render_resource::{Buffer, BindGroup}};

use crate::prelude::Grass;

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct GrassCache {
    pub data: HashMap<Entity, CachedGrassChunk>,
}
#[derive(Debug, Default)]
pub struct CachedGrassChunk {
    pub grass: Grass,
    pub uniform_bindgroup: Option<BindGroup>,
    pub grass_buffer: Option<Buffer>,
    pub transform: GlobalTransform,
}