use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        texture::{CompressedImageFormats, ImageType},
    },
};

pub mod grass;
mod render;
use bevy_inspector_egui::prelude::*;
use grass::Grass;
use warblers_plugin::GRASS_MESH_HANDLE;
pub mod file_loader;
pub mod generator;
pub mod warblers_plugin;
pub mod prelude {
    pub use crate::generator::standard_generator::*;
    pub use crate::grass::*;
    pub use crate::warblers_plugin::WarblersPlugin;
    pub use crate::RegionConfiguration;
    pub use crate::WarblersBundle;
}

#[derive(Bundle)]
pub struct WarblersBundle {
    pub grass: Grass,
    pub grass_mesh: Handle<Mesh>,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass: Default::default(),
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            spatial: Default::default(),
        }
    }
}

#[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Resource, Clone, Reflect)]
#[reflect(Resource)]
pub struct RegionConfiguration {
    pub main_color: Color,
    pub bottom_color: Color,
    pub wind: Vec2,
    pub wind_noise_texture: Handle<Image>,
}
impl FromWorld for RegionConfiguration {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        let img = Image::from_buffer(
            include_bytes!("render/assets/default_noise.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            false,
        )
        .unwrap();
        RegionConfiguration {
            main_color: Color::rgb(0.3, 0.6, 0.0),
            bottom_color: Color::rgb(0.1, 0.3, 0.0),
            wind: Vec2::new(0., 1.0),
            wind_noise_texture: images.add(img),
        }
    }
}
impl ExtractResource for RegionConfiguration {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
