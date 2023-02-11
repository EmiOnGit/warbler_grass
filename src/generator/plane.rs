use super::{GrassGenerator, StandardGeneratorConfig};
use crate::{Grass, GrassBlade};
use bevy::prelude::{Transform, Vec3};
use rand::{rngs::SmallRng, Rng, SeedableRng};
pub struct Plane {
    pub dimensions: Transform,
}
impl GrassGenerator<StandardGeneratorConfig> for Plane {
    fn generate_grass(&self, generator_config: StandardGeneratorConfig) -> Grass {
        let mut rand = if let Some(seed) = generator_config.seed {
            SmallRng::seed_from_u64(seed)
        } else {
            SmallRng::from_entropy()
        };
        let area = self.dimensions.translation.x.abs() * self.dimensions.translation.z.abs();
        let blades_count = (area * generator_config.density) as usize;
        let blades = (0..blades_count)
            .into_iter()
            // generate random values and offset them
            .map(|_| {
                let (x, z, mut height_deviation): (f32, f32, f32) = rand.gen();
                height_deviation =
                    (height_deviation - 0.5) * 2. * generator_config.height_deviation;
                let y = x + z;
                let height = generator_config.height + height_deviation;
                (x, y, z, height)
            })
            // apply plane transformations
            .map(|(x, y, z, height)| {
                let mut point = Vec3::new(x, y, z);
                point = self.dimensions.scale * point;
                point = self.dimensions.rotation * point;
                point = self.dimensions.translation * point;
                (point, height)
            })
            // collect as GrassBlade
            .map(|(position, height)| GrassBlade { position, height })
            .collect();
        Grass(blades)
    }
}
