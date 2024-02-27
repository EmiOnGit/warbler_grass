use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, query::QueryItem},
    prelude::Color,
    render::{
        batching::NoAutomaticBatching, extract_component::ExtractComponent, mesh::Mesh,
        prelude::SpatialBundle, primitives::Aabb, texture::Image,
    },
};

use crate::{
    map::DensityMap,
    map::NormalMap,
    map::YMap,
    warblers_plugin::{DEFAULT_IMAGE_HANDLE, DEFAULT_NORMAL_MAP_HANDLE, GRASS_MESH_HANDLE},
};

/// This [`Bundle`] spawns a grass chunk in the world.
#[derive(Bundle)]
pub struct WarblersBundle {
    /// The [`Mesh`] of the grass blades
    ///
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
    pub grass_mesh: Handle<Mesh>,
    /// An [`YMap`] component
    pub y_map: YMap,
    /// An [`NormalMap`] component
    ///
    /// Defaults to a green 1x1 texture.
    /// You might want to take a look at the
    /// `load_grass` example for that
    pub normal_map: NormalMap,
    /// An [`DensityMap`] component
    pub density_map: DensityMap,
    /// An [`WarblerHeight`] component
    pub height: WarblerHeight,
    /// An [`GrassColor`] component
    pub grass_color: GrassColor,
    /// An [`Aabb`] component
    ///
    /// Note that the Aabb is used to define the world dimensions of the [`DensityMap`] and [`YMap`].
    pub aabb: Aabb,
    pub spatial: SpatialBundle,
    pub no_automatic_batching: NoAutomaticBatching,
}
impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE,
            y_map: DEFAULT_IMAGE_HANDLE.into(),
            normal_map: DEFAULT_NORMAL_MAP_HANDLE.into(),
            density_map: DEFAULT_IMAGE_HANDLE.into(),
            height: WarblerHeight::Uniform(1.),
            grass_color: GrassColor::default(),
            aabb: Aabb::default(),
            spatial: SpatialBundle::default(),
            no_automatic_batching: NoAutomaticBatching,
        }
    }
}
/// The height of the grass blades
///
/// Can be used in Combination with the [`WarblersBundle`] to spawn grass chunks
#[derive(Component, Clone, Debug, PartialEq)]
pub enum WarblerHeight {
    /// Sets the height of the grass blades to a constant value.
    Uniform(f32),
    /// Samples the height from an [`Image`]
    ///
    /// The [`Image`] will be scaled over the plane defined by the [`Aabb`]
    Texture(Handle<Image>),
}
/// Defines the color of the grass blades
#[derive(Component, Clone, ExtractComponent)]
pub struct GrassColor {
    /// The main [Color] of the grass used in your game
    pub main_color: Color,
    /// The bottom [Color] of the grass
    ///
    /// Normally, a darker variant of the main color is choosen to reflect the natural behavior of light
    pub bottom_color: Color,
}
impl Default for GrassColor {
    fn default() -> Self {
        GrassColor {
            main_color: Color::rgb(0.2, 0.5, 0.0),
            bottom_color: Color::rgb(0.1, 0.1, 0.0),
        }
    }
}
impl ExtractComponent for WarblerHeight {
    type QueryData = &'static Self;

    type QueryFilter = ();

    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        match item {
            WarblerHeight::Uniform(_) => Some(item.clone()),
            WarblerHeight::Texture(handle) => Some(WarblerHeight::Texture(handle.clone_weak())),
        }
    }
}
