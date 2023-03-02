use bevy::{
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
    utils::{HashMap, HashSet},
};

use crate::grass_spawner::GrassSpawnerFlags;

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct GrassCache {
    pub data: HashMap<Entity, CachedGrassChunk>,
}

#[derive(Debug, Default)]
pub struct CachedGrassChunk {
    pub uniform_bindgroup: Option<BindGroup>,
    pub instances: Option<Vec<Vec3>>,
    pub instance_buffer: Option<Buffer>,
    pub height_map: Option<BindGroup>,
    pub explicit_y_buffer: Option<BindGroup>,
    pub transform: GlobalTransform,
    pub flags: GrassSpawnerFlags,
}
#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct EntityCache {
    pub entities: HashSet<Entity>,
}
