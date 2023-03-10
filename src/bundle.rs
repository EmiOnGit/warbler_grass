use bevy::{
    prelude::*,
    render::{primitives::Aabb, texture::DEFAULT_IMAGE_HANDLE},
};

use crate::{density_map::DensityMap, height_map::HeightMap, warblers_plugin::GRASS_MESH_HANDLE};

/// This [`Bundle`] spawns a grass chunk in the world.
///
/// This is the recommended way to spawn grass in games.
/// # Note
/// If you only want to input explicit positions of the grass blades you can also use
/// the [`WarblersExplicitBundle`].
#[derive(Bundle)]
pub struct WarblersBundle {
    /// The [`Mesh`] of the grass blades
    /// 
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
    pub grass_mesh: Handle<Mesh>,
    /// An [`HeightMap`] component
    pub height_map: HeightMap,
    /// An [`DensityMap`] component
    pub density_map: DensityMap,
    /// An [`WarblerHeight`] component
    pub height: WarblerHeight,
    /// An [`Aabb`] component
    /// 
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
/// The height of the grass blades
/// 
/// Can be used in Combination with the [`WarblersBundle`] to spawn grass chunks
#[derive(Component, Clone)]
pub enum WarblerHeight {
    /// Sets the height of the grass blades to a constant value.
    Uniform(f32),
    /// Samples the height from an [`Image`]
    /// 
    /// The [`Image`] will be scaled over the plane defined by the [`Aabb`] 
    Texture(Handle<Image>),
}

/// Used to define the positions of all the grass blades explicitly 
/// 
/// Can be used with the [`WarblersExplicitBundle`]
/// 
/// # Example
/// ```rust
/// use warbler_grass::prelude::Grass;
/// use bevy::prelude::Vec3;
/// let mut positions = Vec::with_capacity(100 * 100);
/// for x in 0..100 {
///     for y in 0..100 {
///         positions.push(Vec3::new(x as f32,0., y as f32));
///     }
/// }
/// let height = 2.;
/// 
/// // One way to create grass
/// let grass1 = Grass::new(positions.clone(), height);
/// 
/// // Another way 
/// let grass2 = Grass::from(&positions[..]).with_height(height);
/// assert_eq!(grass1, grass2);
/// ```
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Grass {
    /// The positions of each grass blade defined 
    /// 
    /// The positions are always relative to the entity [`Transform`] component.
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
    /// sets the [`Grass`] height and returns itself after
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}
/// Can be used to create grass from a slice of positions
/// 
/// The height will be set to the default height
impl From<&[Vec3]> for Grass {
    fn from(value: &[Vec3]) -> Self {
        Self {
            positions: value.into(),
            height: Default::default(),
        }
    }
}
/// A bundle spawning a grass chunk in the world
///
/// It uses explicit positions of all grass blades to generate the them
/// For an example take a look at the `load_explicit` example
#[derive(Bundle)]
pub struct WarblersExplicitBundle {
    /// The [`Mesh`] of the grass blades
    /// 
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
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
