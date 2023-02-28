use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        texture::{CompressedImageFormats, ImageType},
    },
};

pub mod grass;
pub mod grass_spawner;
pub mod height_map;
mod render;
use bevy_inspector_egui::prelude::*;
use grass::Grass;
use grass_spawner::GrassSpawner;
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

/// A component bundle for a chunk of grass.
///
/// Note that each position of a [`GrassBlade`](crate::prelude::GrassBlade) is also relative to the [`Transform`] component of the entity
#[derive(Bundle)]
pub struct WarblersBundle {
    pub grass_spawner: GrassSpawner,
    /// The [`Mesh`] used to render each grassblade.
    ///
    /// The mesh can be changed to however needed,
    /// however note that the lowest vertex of the mesh should be around y=0
    /// in most cases.
    pub grass_mesh: Handle<Mesh>,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_spawner: Default::default(),
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            spatial: Default::default(),
        }
    }
}
/// A [resource](bevy::prelude::Resource) used to globally define parameters about the grass.
///
/// A default [`RegionConfiguration`] is inserted by the [`WarblersPlugin`](crate::warblers_plugin::WarblersPlugin).
#[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Resource, Clone, Reflect, ExtractResource)]
#[reflect(Resource)]
pub struct RegionConfiguration {
    /// The main [Color] of the grass used in your game.
    pub main_color: Color,
    /// The bottom [Color] of the grass.
    ///
    /// Normally, a darker variant of the main color is choosen to reflect the natural behavior of light.
    pub bottom_color: Color,
    /// The direction and strength of wind.
    ///
    /// The direction of the wind is on the x,z plane.
    ///
    /// Be aware that the strength of the wind is controlled by the (euclidean) norm of the vector.
    /// If you want to turn of wind in your game, you can just set this to [`Vec2::ZERO`]
    pub wind: Vec2,
    /// The texture used to animate the wind on the grass.
    ///
    /// Most likely you don't need to change that unless you want your wind to feel different.
    /// If you decide to swap it, note that you want the texture to be tileable,
    /// also currently only the red and green chanel are used
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
            main_color: Color::rgb(0.2, 0.5, 0.0),
            bottom_color: Color::rgb(0.1, 0.1, 0.0),
            wind: Vec2::new(0., 1.0),
            wind_noise_texture: images.add(img),
        }
    }
}
