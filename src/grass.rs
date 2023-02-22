use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, primitives::Aabb, render_resource::ShaderType},
};
use bytemuck::{Pod, Zeroable};

/// Representation of a single grassblade
#[derive(Copy, Clone, Debug, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GrassBlade {
    /// The position of the [GrassBlade].
    ///
    /// Note that the end position is also relative to the [`Transform`] of the entity containing the blades.
    pub position: Vec3,
    /// The height of the grass blade.
    ///
    /// Internally scales the the grass mesh in the y direction
    pub height: f32,
}

/// A collection of grassblades to be extracted later into the render world
#[derive(Clone, Debug, Component, Default)]
pub struct Grass {
    pub instances: Vec<GrassBlade>,
}
impl Grass {
    pub fn new(instances: Vec<GrassBlade>) -> Self {
        Grass { instances }
    }
    /// Calculates an [`Aabb`] box which contains all grass blades in self.
    ///
    /// This can be used to check if the grass is in the camera view
    pub fn calculate_aabb(&self) -> Aabb {
        let mut outer = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        let mut inner = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        self.instances
            .iter()
            .map(|blade| (blade.position, blade.height))
            .for_each(|(blade_pos, height)| {
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
/// To calculate frustum culling we need the [Aabb] box of the entity
///
/// Note that it is in the responsabilty of the user to minimize the [Aabb] boxes of the chunks if high performance is needed
pub(crate) fn add_aabb_box_to_grass(
    mut commands: Commands,
    grasses: Query<(Entity, &Grass), Added<Grass>>,
) {
    for (e, grass) in grasses.iter() {
        let aabb = grass.calculate_aabb();
        commands.entity(e).insert(aabb);
    }
}
