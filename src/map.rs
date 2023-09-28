//! Contains the [`YMap`](crate::maps::YMap) and [`DensityMap`](crate::maps::DensityMap) component

use bevy::{
    asset::Handle,
    ecs::{component::Component, query::QueryItem},
    reflect::Reflect,
    render::{extract_component::ExtractComponent, texture::Image},
};

/// The y-map defining the y position of the grass blades.
///
/// A [`YMap`] is usually called a heightmap in game dev.
/// Since it was confused with the [`WarblerHeight`](crate::prelude::WarblerHeight),
/// which controls the actual height of the grass blades, we decided to give it another name
///
/// The maximum height of the y-map is controlled by the height of the [`Aabb`](bevy::render::primitives::Aabb).
/// The y-map texture will be scaled over all grass blades.
///
/// For a simple example, take a look at the [`load_grass`](https://github.com/emiongit/warbler_grass/latest/example/load_grass.rs) example
#[derive(Reflect, Clone, Component)]
pub struct YMap {
    pub y_map: Handle<Image>,
}
impl YMap {
    /// Creates a new `YMap`
    pub fn new(y_map: Handle<Image>) -> Self {
        YMap { y_map }
    }
}
impl From<Handle<Image>> for YMap {
    fn from(value: Handle<Image>) -> Self {
        YMap { y_map: value }
    }
}
impl ExtractComponent for YMap {
    type Query = &'static Self;

    type Filter = ();

    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        Some(YMap {
            y_map: item.y_map.clone_weak(),
        })
    }
}

#[derive(Reflect, Clone, Component)]
pub struct NormalMap {
    pub normal_map: Handle<Image>,
}
impl NormalMap {
    /// Creates a new `NormalMap`
    pub fn new(normal_map: Handle<Image>) -> Self {
        NormalMap { normal_map }
    }
}
impl From<Handle<Image>> for NormalMap {
    fn from(value: Handle<Image>) -> Self {
        NormalMap { normal_map: value }
    }
}
impl ExtractComponent for NormalMap {
    type Query = &'static Self;

    type Filter = ();

    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        Some(NormalMap {
            normal_map: item.normal_map.clone_weak(),
        })
    }
}

/// The density map defines the density of grass blades at a given positions.
///
/// The area covered by the density map is defined by the area of the [`Aabb`](bevy::render::primitives::Aabb) component.
/// The [`DensityMap`] texture will be scaled over the complete area
/// Often a small density map is enough to cover big areas!
///
/// For a simple example, take a look at the [`load_grass`](https://github.com/emiongit/warbler_grass/latest/example/load_grass.rs) example.
#[derive(Reflect, Clone, Component)]
pub struct DensityMap {
    /// The density map of the grass chunk.
    ///
    /// Should be ideally gray scale image for memory efficency.
    /// White pixels corresponds to dense areas.
    /// Black pixels correspond to sparse areas.
    pub density_map: Handle<Image>,
    /// The density of the grass.
    ///
    /// If the density is high, more grass is spawned in a dense area.
    /// The density should always be positiv
    pub density: f32,
}
impl DensityMap {
    /// Creates a new `DensityMap`
    pub fn new(density_map: Handle<Image>, density: f32) -> Self {
        if density < 0. {
            bevy::log::warn!("DensityMap was created with a negative density of {density}");
        }
        DensityMap {
            density_map,
            density,
        }
    }
}
/// A density map can be created from the image alone
///
/// The density will be set to 1
impl From<Handle<Image>> for DensityMap {
    fn from(value: Handle<Image>) -> Self {
        DensityMap {
            density_map: value,
            density: 1.,
        }
    }
}
