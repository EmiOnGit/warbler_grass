use bevy::{
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
    utils::HashMap,
};

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
#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct EntityCache {
    pub entities: Vec<Entity>
}
