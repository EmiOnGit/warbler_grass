//! Contains the implementation of the [`DensityMap`] component

use bevy::prelude::*;
/// The density map defining the density of grass at specific positions.
/// White pixels corresponds to dense areas.
///
/// Usually, this component is used in the [`WarblersBundle`](crate::bundle::WarblersBundle)
///
/// The area covered by the density map is defined by the area of the [`Aabb`](bevy::render::primitives::Aabb) component
///
/// The [`DensityMap`] texture will be scaled over the complete area
/// Often a small density map is enough to cover big areas!
///
/// For a simple example, take a look at the `load_grass` example.
#[derive(Reflect, Clone, Component)]
pub struct DensityMap {
    /// The density map of the grass chunk.
    ///
    /// Should be a gray scale image
    pub density_map: Handle<Image>,
    /// The density of the grass.
    ///
    /// If the density is high, more grass is spawned in a dense area
    pub density: f32,
}
/// A density map can be created from the image alone
///
/// The density will be set to 1
impl From<Handle<Image>> for DensityMap {
    fn from(value: Handle<Image>) -> Self {
        DensityMap {
            density_map: value,
            density: 1.,
        }
    }
}
