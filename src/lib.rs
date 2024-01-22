//! [![crates.io](https://img.shields.io/badge/crates.io-v0.4-orange)](https://crates.io/crates/warbler_grass)
//!
//! A bevy plugin for easily creating million of grass blades in your game.
//! The crate heavily uses instanced rendering to render as much grass as possible.
//! More information can be found on the [`github repository`](https://github.com/EmiOnGit/warbler_grass)
use bevy::{
    asset::{Assets, Handle},
    ecs::prelude::{FromWorld, ReflectResource, Resource, World},
    math::Vec2,
    reflect::Reflect,
    render::{
        extract_resource::ExtractResource,
        prelude::Image,
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
};

pub mod bundle;
pub mod dithering;

pub mod diagnostic;
#[cfg(feature = "editor")]
pub mod editor;

pub mod map;

mod render;
pub mod warblers_plugin;
pub mod prelude {
    pub use crate::bundle::*;
    pub use crate::map::*;
    pub use crate::warblers_plugin::WarblersPlugin;
    pub use crate::GrassConfiguration;
}

/// A [resource](bevy::prelude::Resource) used to globally define parameters about the grass.
///
/// A default [`GrassConfiguration`] is inserted by the [`WarblersPlugin`](crate::warblers_plugin::WarblersPlugin).
#[derive(Resource, Clone, Reflect, ExtractResource)]
#[reflect(Resource)]
pub struct GrassConfiguration {
    /// The direction and strength of wind.
    ///
    /// The direction of the wind is on the x,z plane.
    ///
    /// Be aware that the strength of the wind is controlled by the length of the vector.
    /// If you want to turn of wind in your game, you can just set this to `Vec2::ZERO`
    ///
    /// If you want to change the generel look of the wind and not only the wind direction/ speed
    /// you can also change the noise texture used for the wind that is stored in the
    /// [`GrassNoiseTexture`] resource
    pub wind: Vec2,
}
impl Default for GrassConfiguration {
    fn default() -> Self {
        GrassConfiguration {
            wind: Vec2::new(1.0, 1.0),
        }
    }
}

/// The texture used to animate the wind on the grass.
///
/// Most likely you don't need to change that unless you want your wind to "feel" different.
/// If you decide to swap it, note that you want the texture to be tileable,
#[derive(Resource, ExtractResource)]
pub struct GrassNoiseTexture(Handle<Image>);
impl Clone for GrassNoiseTexture {
    fn clone(&self) -> Self {
        Self(self.0.clone_weak())
    }
}

impl FromWorld for GrassNoiseTexture {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        let img = Image::from_buffer(
            include_bytes!("render/assets/default_noise.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            false,
            ImageSampler::Default,
        )
        .unwrap();
        GrassNoiseTexture(images.add(img))
    }
}
