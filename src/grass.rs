use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct GrassBlade {
    /// The position of the [GrassBlade].
    ///
    /// Note that the end position is also relative to the [`Transform`] of the entity containing the blades.
    pub position: Vec3,
    /// The height of the grass blade.
    ///
    /// Internally scales the the grass mesh in the y direction
    pub height: f32,
}
