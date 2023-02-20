use bevy::{prelude::*, render::{render_resource::ShaderType, extract_component::ExtractComponent, primitives::Aabb}};
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
    /// Calculates an [`Aabb`] box which contains all grass blades in self.
    /// 
    /// This can be used to check if the grass is in the camera view
    pub fn calculate_aabb(&self) -> Aabb {
        let mut outer = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        let mut inner = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        self.instances.iter()
            .map(|blade|  (blade.position,blade.height))
            .for_each(|(blade_pos,height)| {
                inner = inner.min(blade_pos);
                outer = outer.max(blade_pos + Vec3::Y * height);
            });
        Aabb::from_min_max(inner, outer)
    }
}
impl ExtractComponent for Grass {
    type Query = &'static Grass;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Self {
        item.clone()
    }
}
pub(crate) fn add_aabb_box_to_grass(
    mut commands: Commands,
    grasses: Query<(Entity, &Grass), Added<Grass>>
) {
    for (e, grass) in grasses.iter() {
        let aabb = grass.calculate_aabb();
        commands.entity(e).insert(aabb);
    }

}