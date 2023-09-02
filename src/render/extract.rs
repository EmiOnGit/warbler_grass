use crate::{dithering::DitheredBuffer, map::YMap};
use bevy::{
    prelude::*,
    render::{primitives::Aabb, Extract},
};

/// Extracts the grass data of entities spawned with the [`WarblersBundle`](crate::bundle::WarblersBundle) into the render world
///
/// The extraction only happens on change or creation of the entity,
/// so it normally doesn't come at a high performance cost
#[allow(clippy::type_complexity)]
pub(crate) fn extract_grass(
    mut commands: Commands,
    grass_spawner: Extract<Query<(Entity, &Handle<DitheredBuffer>, &Aabb)>>,
) {
    let mut values = Vec::new();
    for (entity, dithered, aabb) in grass_spawner.iter() {
        values.push((entity, (dithered.clone(), *aabb)));
    }
    commands.insert_or_spawn_batch(values);
}

pub(crate) fn extract_aabb(
    mut commands: Commands,
    aabbs: Extract<Query<(Entity, &Aabb), With<YMap>>>,
) {
    let mut values = Vec::new();
    for (e, aabb) in aabbs.iter() {
        values.push((e, *aabb));
    }
    commands.insert_or_spawn_batch(values);
}
