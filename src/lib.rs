use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        texture::{CompressedImageFormats, ImageType},
    },
};

pub mod bundle;
pub mod density_map;
pub mod dithering;
#[cfg(feature = "editor")]
pub mod editor;
pub mod height_map;
mod render;
pub mod warblers_plugin;
pub mod prelude {
    pub use crate::bundle::*;
    pub use crate::density_map::DensityMap;
    pub use crate::height_map::HeightMap;
    pub use crate::warblers_plugin::WarblersPlugin;
    pub use crate::GrassConfiguration;
}

/// A [resource](bevy::prelude::Resource) used to globally define parameters about the grass.
///
/// A default [`GrassConfiguration`] is inserted by the [`WarblersPlugin`](crate::warblers_plugin::WarblersPlugin).
#[derive(Resource, Clone, Reflect, ExtractResource)]
#[reflect(Resource)]
pub struct GrassConfiguration {
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
    /// Be aware that the strength of the wind is controlled by the length of the vector.
    /// If you want to turn of wind in your game, you can just set this to [`Vec2::ZERO`]
    pub wind: Vec2,
    /// The texture used to animate the wind on the grass.
    ///
    /// Most likely you don't need to change that unless you want your wind to "feel" different.
    /// If you decide to swap it, note that you want the texture to be tileable,
    pub wind_noise_texture: Handle<Image>,
}
impl FromWorld for GrassConfiguration {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        let img = Image::from_buffer(
            include_bytes!("render/assets/default_noise.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            false,
        )
        .unwrap();
        GrassConfiguration {
            main_color: Color::rgb(0.2, 0.5, 0.0),
            bottom_color: Color::rgb(0.1, 0.1, 0.0),
            wind: Vec2::new(0., 1.0),
            wind_noise_texture: images.add(img),
        }
    }
}
