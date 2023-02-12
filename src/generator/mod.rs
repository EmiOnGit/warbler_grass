pub mod grass_field;
pub mod plane;
use crate::Grass;

pub mod standard_generator {
    pub use super::grass_field::GrassFieldGenerator;
    pub use super::plane::Plane;
    pub use super::StandardGeneratorConfig;
    pub use super::GrassGenerator;
}
pub trait GrassGenerator<Config> {
    fn generate_grass(&self, generator_config: Config) -> Grass;
}
#[derive(Debug, Clone)]
pub struct StandardGeneratorConfig {
    /// Density of the grass generated.
    /// The higher the density the more grass will be spawned per area
    pub density: f32,
    /// The mean height of the grass blades created by the generator
    pub height: f32,
    /// The deviation of the blade heights.
    ///
    /// If you want no deviation in your blades, you can set it to 0
    pub height_deviation: f32,
    /// The seed used for the random number generator, which calculates height, x and z coordinates of the grass blades
    ///
    /// If you want the grass to look always the same you can set a seed.
    /// If [None] is used, the seed is calculated from the internal random generator of the running OS
    pub seed: Option<u64>,
}

impl Default for StandardGeneratorConfig {
    fn default() -> Self {
        Self {
            density: 20.,
            height: 2.,
            height_deviation: 0.5,
            seed: None,
        }
    }
}
