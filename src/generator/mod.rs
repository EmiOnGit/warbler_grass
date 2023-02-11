pub mod plane;

use crate::Grass;

pub trait GrassGenerator<Config> {
    fn generate(&self, generator_config: Config) -> Grass;
}

pub struct StandardGeneratorConfig {
    pub density: f32,
    pub height: f32,
    pub height_deviation: f32,
    pub seed: Option<u64>,
}

impl Default for StandardGeneratorConfig {
    fn default() -> Self {
        Self { density: 20., height: 2., height_deviation: 0.5, seed: None }
    }
}