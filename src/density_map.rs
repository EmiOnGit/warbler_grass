use bevy::prelude::*;

#[derive(Reflect, Clone, Component)]
pub struct DensityMap {
    /// The density map of the grass chunk
    ///
    /// Should be a Gray scale image
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
