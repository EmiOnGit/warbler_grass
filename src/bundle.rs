use bevy::{
    prelude::*,
    render::{primitives::Aabb, texture::DEFAULT_IMAGE_HANDLE},
};

use crate::{density_map::DensityMap, height_map::HeightMap, warblers_plugin::GRASS_MESH_HANDLE};

#[derive(Bundle)]
pub struct WarblersBundle {
    pub grass_mesh: Handle<Mesh>,
    pub height_map: HeightMap,
    pub density_map: DensityMap,
    pub height: WarblerHeight,
    pub aabb: Aabb,
    #[bundle]
    pub spatial: SpatialBundle,
}
impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            height_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            density_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            height: WarblerHeight::Uniform(1.),
            aabb: Default::default(),
            spatial: Default::default(),
        }
    }
}
#[derive(Component, Clone)]
pub enum WarblerHeight {
    Uniform(f32),
    Texture(Handle<Image>),
}
