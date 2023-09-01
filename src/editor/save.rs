use std::path::PathBuf;

use bevy::prelude::{info, warn, Assets, DetectChanges, Image, Query, Res, Resource};

use crate::prelude::{DensityMap, HeightMap, WarblerHeight};

use super::{ray_cast::SelectedMap, ActiveEditorChunk};

pub fn check_for_save_files(
    saver: Res<ImageSaver>,
    active_map: Res<SelectedMap>,
    active_chunk: Res<ActiveEditorChunk>,
    q: Query<(
        Option<&DensityMap>,
        Option<&HeightMap>,
        Option<&WarblerHeight>,
    )>,
    images: Res<Assets<Image>>,
) {
    if saver.is_changed() {
        let Some(entity) = active_chunk.0 else {
            return;
        };
        if let Ok((density_map, height_map, warbler_height)) = q.get(entity) {
            let image_handle = match *active_map {
                SelectedMap::HeightMap => {
                    let Some(height_map) = height_map else {
                        return;
                    };
                    &height_map.height_map
                }
                SelectedMap::DensityMap => {
                    let Some(density_map) = density_map else {
                        return;
                    };
                    &density_map.density_map
                }
                SelectedMap::HeightsMap => {
                    let Some(warbler_height) = warbler_height else {
                        return;
                    };
                    let WarblerHeight::Texture(tex) = warbler_height else {
                        return;
                    };
                    tex
                }
            };
            let Some(image) = images.get(image_handle) else {
                info!("Image was not yet loaded. Saving failed");
                return;
            };
            match saver.save(image) {
                Ok(_) => info!("Successfully saved image to {:?}", saver.path),
                Err(e) => warn!("Failed saving the image with error {e:?}"),
            }
        }
    }
}
#[derive(Resource, Default)]
pub struct ImageSaver {
    pub path: Option<PathBuf>,
}

impl ImageSaver {
    pub fn save(&self, image: &Image) -> Result<(), SaveError> {
        let Some(path) = self.path.as_ref() else {
            return Err(SaveError::NoPathFound);
        };
        let Ok(image) = image.clone().try_into_dynamic() else {
            return Err(SaveError::WrongImageFormat);
        };
        let image = image.to_luma8();

        let error = image.save(path);
        // Isn't best practise but allows us to not import the image crate as dep
        if let Err(e) = error {
            let message = e.to_string();
            if message.contains("The image format could not be determined") {
                warn!("Have you included a file extension?")
            }
            return Err(SaveError::ImageError(message));
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum SaveError {
    NoPathFound,
    ImageError(String),
    WrongImageFormat,
}
