//! Contains the implementation of the [`YMap`] component
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
/// The maximum height of the height map is controlled by the height of the [`Aabb`](bevy::render::primitives::Aabb).
/// The y-map texture will be scaled over all grass blades.
///
/// For a simple example, take a look at the [`load_grass`](https://github.com/emiongit/warbler_grass/latest/example/load_grass.rs) example
#[derive(Reflect, Clone, Component)]
pub struct YMap {
    pub y_map: Handle<Image>,
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
