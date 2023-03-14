use bevy::{ecs::prelude::*, render::primitives::Aabb, prelude::Vec3};

use crate::prelude::Grass;
pub fn add_aabb_to_explicit(
    mut commands: Commands,
    grasses: Query<(Entity, &Grass), Or<(Changed<Grass>, Changed<Aabb>)>>,
    mut added_last_frame: Local<Vec<Entity>>
) {
    let mut added = Vec::new();
    for (e, grass) in grasses.iter().filter(|(e,_)| !added_last_frame.contains(e)) {
        let mut outer = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        let mut inner = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        for position in grass.positions.iter() {
            inner = inner.min(*position);
            outer = outer.max(*position + Vec3::Y * grass.height);
        } 
        added.push(e);
        let aabb = Aabb::from_min_max(inner, outer);
        commands.entity(e).insert(aabb);
    }
    *added_last_frame = added;
}