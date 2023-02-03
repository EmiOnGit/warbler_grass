use bevy::{prelude::*, render::extract_component::ExtractComponent};
use bytemuck::{Pod, Zeroable};
mod render;
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
