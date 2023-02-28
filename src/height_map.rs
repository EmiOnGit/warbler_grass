use bevy::prelude::*;
use bevy::render::primitives::Aabb;

pub struct HeightMap {
    pub height_map: Handle<Image>,
    pub aabb: Aabb,
}
