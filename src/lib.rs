use bevy::{prelude::*, render::{extract_component::ExtractComponent, extract_resource::ExtractResource}};
use bytemuck::{Pod, Zeroable};
mod render;
use bevy_inspector_egui::prelude::*;

pub mod warblers_plugin;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone, Debug, Component)]
pub struct GrassData(pub Vec<GrassBlade>);

impl ExtractComponent for GrassData {
    type Query = &'static GrassData;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Self {
        item.clone()
    }
}
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct GrassBlade {
    pub position: Vec3,
    pub height: f32,
}

#[derive(Resource, Clone, Reflect, InspectorOptions)] 
#[reflect(Resource, InspectorOptions)]
pub struct RegionConfig {
    name: String,
    pub color: Color,
}
impl Default for RegionConfig {
    fn default() -> Self {
        RegionConfig { name: "Default Config".to_string(), color: Color::rgb(0.3, 0.5, 0.0) }
    }
}
impl ExtractResource for RegionConfig {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
