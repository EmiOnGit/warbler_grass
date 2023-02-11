use bevy::prelude::{Handle, Vec3};
use rand::{rngs::SmallRng, SeedableRng, Rng};

use crate::{Grass, GrassBlade};

use super::{file_loader::GrassDataAsset, StandardGeneratorConfig, GrassGenerator};

pub struct GrassDataAssetGenerator<'a> {
    pub data: &'a GrassDataAsset,
}

impl GrassGenerator<StandardGeneratorConfig> for GrassDataAssetGenerator<'_> {
    fn generate(&self, generator_config: StandardGeneratorConfig) -> crate::Grass {
        let mut rand  = if let Some(seed) = generator_config.seed {
            SmallRng::seed_from_u64(seed)
        } else {
            SmallRng::from_entropy()
        };
        let mut blades: Vec<GrassBlade> = Vec::new();
        for [y, x,z,width,height] in self.data.0.iter() {
            if *y == 0 {
                continue
            }
            let area = width * height;
            let blades_count = area * generator_config.density as u16;
            let rect_blades = (0..blades_count).into_iter()
                // generate random values and offset them
                .map(|_| {
                    let (x_delta, z_delta, mut height_deviation): (f32,f32, f32) = rand.gen();
                    height_deviation = (height_deviation - 0.5) * 2. * generator_config.height_deviation;
                    let grass_height = generator_config.height + height_deviation;
                    (*x as f32 + x_delta * *width as f32,*y as f32,*z as f32+ z_delta * *height as f32 ,grass_height) 
                })
                // apply plane transformations
                .map(|(x,y,z,height)| {
                    let point = Vec3::new(x , y, z);
                    (point, height)
                })
                // collect as GrassBlade
                .map(|(position, height)| GrassBlade {
                    position,
                    height,
                });
            blades.extend(rect_blades);
        }
        Grass(blades)
    }
}