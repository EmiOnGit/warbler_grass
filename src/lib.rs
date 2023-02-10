use bevy::{prelude::*, render::{extract_component::ExtractComponent, extract_resource::ExtractResource, view::NoFrustumCulling, render_resource::{Buffer, PrimitiveTopology}, mesh::Indices}};
use bytemuck::{Pod, Zeroable};
mod render;
use bevy_inspector_egui::prelude::*;
use warblers_plugin::GRASS_MESH_HANDLE;

pub mod warblers_plugin;
#[derive(Bundle)]
pub struct WarblersBundle{
    pub grass_data: GrassData,
    // pub grass_mesh: Handle<Mesh>,
    pub transform: Transform,
    pub no_frustum_calling: NoFrustumCulling,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
impl Default for WarblersBundle {
    fn default() -> Self {
        Self { no_frustum_calling: NoFrustumCulling, grass_data: Default::default(), transform: Default::default(), global_transform: Default::default(), visibility: Default::default(), computed_visibility: Default::default() }
    }
}

#[derive(Clone, Debug, Component, Default)]
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
#[cfg_attr(feature = "debug", derive(InspectorOptions))]

#[derive(Resource, Clone, Reflect)] 
#[reflect(Resource)]
pub struct RegionConfig {
    pub color: Color,
    pub wind: Vec2,
}

impl Default for RegionConfig {
    fn default() -> Self {
        RegionConfig { 
            color: Color::rgb(0.3, 0.6, 0.0),
            wind: Vec2::new(0.6,0.)
        }
    }
}
impl ExtractResource for RegionConfig {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
