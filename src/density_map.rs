use bevy::prelude::*;
/// The density map defining the density of grass at specific positions.
/// White pixels corresponds to dense areas.
///
///
/// The game area covered by the density map is defined by the area of the [`Aabb`](bevy::render::primitives::Aabb)
/// inserted as [`Component`].
///
/// The density map texture will be scaled over the complete area.
/// Often a small density map is enough to cover big areas!
///
/// For a simple example, take a look at the `load_grass` example.
///
/// # Note
/// Internally, the grass is spawned using dithering of the density map.
/// This is rather cheap to calculate but can become still expensive for large areas.
/// This can quickly become a bottleneck if you change the density map each frame!
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
impl From<Handle<Image>> for DensityMap {
    fn from(value: Handle<Image>) -> Self {
        DensityMap {
            density_map: value,
            density: 1.,
        }
    }
}
