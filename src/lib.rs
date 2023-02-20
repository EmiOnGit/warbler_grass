use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, extract_resource::ExtractResource, texture::{CompressedImageFormats, ImageType},
    }, sprite::Material2d,
};
use bytemuck::{Pod, Zeroable};
mod render;
use bevy_inspector_egui::prelude::*;
use warblers_plugin::GRASS_MESH_HANDLE;
pub mod file_loader;
pub mod generator;
pub mod warblers_plugin;
pub mod prelude {
    pub use crate::generator::standard_generator::*;
    pub use crate::warblers_plugin::WarblersPlugin;
    pub use crate::RegionConfiguration;
    pub use crate::WarblersBundle;
}

#[derive(Bundle)]
pub struct WarblersBundle {
    pub grass: Grass,
    pub grass_mesh: Handle<Mesh>,
    pub transform: Transform,
    // pub no_frustum_calling: NoFrustumCulling,
    // pub bounds: Aabb,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass: Default::default(),
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            transform: Default::default(),
            // bounds: Aabb { center: Vec3A::new(10.,1.,100.), half_extents: Vec3A::new(10.,1.,100.) },
            // no_frustum_calling: NoFrustumCulling,
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
    pub wind_noise_texture: Handle<Image>,
}
impl FromWorld for RegionConfiguration {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
            let img = Image::from_buffer(
                include_bytes!("render/assets/default_noise.png"),
                ImageType::Extension("png"),
                CompressedImageFormats::default(),
                false,
            )
            .unwrap();
        RegionConfiguration {
            color: Color::rgb(0.3, 0.6, 0.0),
            wind: Vec2::new(0., 1.0),
            wind_noise_texture: images.add(img),
        }
    }
}
impl ExtractResource for RegionConfiguration {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
