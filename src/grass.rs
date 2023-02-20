use bevy::{prelude::*, render::{render_resource::ShaderType, extract_component::ExtractComponent}};
use bytemuck::{Pod, Zeroable};
/// An single grassblade, with the lower part at `position`
#[derive(Copy, Clone, Debug, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GrassBlade {
    /// The position of the lower part of the mesh.(At least for the default grass mesh)
    pub position: Vec3,
    /// The height of the grass blade. Internally scales the the grass mesh in the y direction
    pub height: f32,
}

/// A collection of grassblades to be extracted later into the render world
#[derive(Clone, Debug, Component, Default)]
pub struct Grass{
    pub instances: Vec<GrassBlade>
}
impl Grass {
    pub fn new(instances: Vec<GrassBlade>) -> Self {
        Grass {
            instances
        }
    }
}
impl ExtractComponent for Grass {
    type Query = &'static Grass;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Self {
        item.clone()
    }
}
