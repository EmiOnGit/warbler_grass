use bevy::{
    prelude::*,
    render::{primitives::Aabb, texture::DEFAULT_IMAGE_HANDLE},
};

use crate::{density_map::DensityMap, height_map::HeightMap, warblers_plugin::GRASS_MESH_HANDLE};

/// A bundle spawning a grass chunk in the world.
///
/// This is the recommended way to spawn grass in games.
/// # Note
/// If you only want to input explicit positions of the grass blades you can also use
/// the [`WarblersExplicitBundle`].
#[derive(Bundle)]
pub struct WarblersBundle {
    /// The mesh of the grass blades.
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
    pub grass_mesh: Handle<Mesh>,
    pub height_map: HeightMap,
    pub density_map: DensityMap,
    pub height: WarblerHeight,
    /// Note that the Aabb is used to define the world dimensions of the [`DensityMap`] and [`HeightMap`].
    pub aabb: Aabb,
    #[bundle]
    pub spatial: SpatialBundle,
}
impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            height_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            density_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            height: WarblerHeight::Uniform(1.),
            aabb: Default::default(),
            spatial: Default::default(),
        }
    }
}
#[derive(Component, Clone)]
pub enum WarblerHeight {
    /// Sets the height of the grass blades all to the same height
    Uniform(f32),
    /// Currently not supported
    Texture(Handle<Image>),
}

/// Used to define explicitly the positions of all the grass blades.
#[derive(Component, Clone)]
pub struct Grass {
    /// The positions defined here are relative to the entity [`Transform`] component
    pub positions: Vec<Vec3>,
    /// The height of the grass blades
    pub height: f32,
}
impl Default for Grass {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            height: 1.,
        }
    }
}
impl Grass {
    /// Creates a new [`Grass`] instance
    pub fn new(positions: Vec<Vec3>, height: f32) -> Self {
        Grass { positions, height }
    }
}
/// Can be used to easily create grass from the positions.
/// The height will be set to the default height
impl From<&[Vec3]> for Grass {
    fn from(value: &[Vec3]) -> Self {
        Self {
            positions: value.into(),
            height: Default::default(),
        }
    }
}
/// A bundle spawning a grass chunk in the world.
///
/// It uses explicit positions of all grass blades to generate the them.
/// For an example take a look at the `load_explicit` example
///
/// # Note
/// Consider using the [`WarblersBundle`] instead as it has a couple of advantages over explicit positions.
#[derive(Bundle)]
pub struct WarblersExplicitBundle {
    /// The mesh used to draw the grass blades
    pub grass_mesh: Handle<Mesh>,
    /// The explicit positions of the grass blades
    pub grass: Grass,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for WarblersExplicitBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            grass: Grass::default(),
            spatial: Default::default(),
        }
    }
}
