use bevy::prelude::*;
#[derive(Reflect, Clone)]
pub struct DensityMap {
    /// The density map of the grass chunk
    ///
    /// Should be a Gray scale image
    pub density_map: Handle<Image>,
    /// The footprint describes how far apart the grass should be.
    pub footprint: f32,
    /// Describes how far the density map should reach
    pub span_xz: Vec2,
    /// If the positions should be influenced by noise.
    ///
    /// Since we generate the positions out using ordered dithering,
    /// the positions are placed in a grid like structure without noise.
    /// For noise the blue channel from the wind noise texture is used.
    pub noise: bool,
}
