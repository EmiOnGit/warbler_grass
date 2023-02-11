use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, extract_resource::ExtractResource,
        view::NoFrustumCulling,
    },
};
use bytemuck::{Pod, Zeroable};
mod render;
use bevy_inspector_egui::prelude::*;
use warblers_plugin::GRASS_MESH_HANDLE;
pub mod generator;
pub mod warblers_plugin;

#[derive(Bundle)]
pub struct WarblersBundle {
    pub grass_data: Grass,
    pub grass_mesh: Handle<Mesh>,
    pub transform: Transform,
    pub no_frustum_calling: NoFrustumCulling,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_data: Default::default(),
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            transform: Default::default(),
            no_frustum_calling: NoFrustumCulling,
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Component, Default)]
pub struct Grass(pub Vec<GrassBlade>);

impl ExtractComponent for Grass {
    type Query = &'static Grass;
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
pub struct RegionConfiguration {
    pub color: Color,
    pub wind: Vec2,
}

impl Default for RegionConfiguration {
    fn default() -> Self {
        RegionConfiguration {
            color: Color::rgb(0.3, 0.6, 0.0),
            wind: Vec2::new(0.6, 0.),
        }
    }
}
impl ExtractResource for RegionConfiguration {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
