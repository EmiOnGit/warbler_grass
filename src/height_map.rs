//! Contains the implementation of the [`HeightMap`] component

use bevy::prelude::*;
/// The height map defining the y position of the grass blades.
///
/// Usually, this component is used in the [`WarblersBundle`](crate::bundle::WarblersBundle)
///
/// The maximum height of the height map is controlled by the height of the [`Aabb`](bevy::render::primitives::Aabb)
/// inserted as [`Component`] and can be changed at runtime.
///
/// The height map texture will be scaled over all grassblades.
/// It is recommended to use a rather small heightmap if you don't need much detail
///
/// For a simple example, take a look at the `load_grass` example
#[derive(Reflect, Clone, Component)]
pub struct HeightMap {
    pub height_map: Handle<Image>,
}
impl From<Handle<Image>> for HeightMap {
    fn from(value: Handle<Image>) -> Self {
        HeightMap { height_map: value }
    }
}
