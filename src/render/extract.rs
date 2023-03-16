use super::cache::{CachedGrassChunk, GrassCache};
use crate::{bundle::Grass, dithering::DitheredBuffer, height_map::HeightMap};
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
    grass_spawner: Extract<
        Query<(Entity, &Handle<DitheredBuffer>, &Aabb), Changed<Handle<DitheredBuffer>>>,
    >,
    mut grass_cache: ResMut<GrassCache>,
) {
    let mut values = Vec::new();
    for (entity, dithered, aabb) in grass_spawner.iter() {
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.dither_handle = Some(dithered.clone());
        values.push((entity, (dithered.clone(), *aabb)));
    }
    commands.insert_or_spawn_batch(values);
}
/// Extracts the grass data of entities spawned with the [`WarblersExplicitBundle`](crate::bundle::WarblersExplicitBundle) into the render world
///
/// The extraction only happens on change or creation of the entity,
/// so it normally doesn't come at a high performance cost
#[allow(clippy::type_complexity)]
pub(crate) fn extract_grass_positions(
    mut commands: Commands,
    grass_spawner: Extract<
        Query<
            (Entity, &Grass, &Aabb), // , Or<(Changed<Grass>, Changed<Aabb>)>
        >,
    >,
    mut grass_cache: ResMut<GrassCache>,
) {
    let mut values = Vec::new();

    for (entity, grass, aabb) in grass_spawner.iter() {
        if !grass_cache.contains_key(&entity) {
            grass_cache.insert(entity, CachedGrassChunk::default());
        }
        values.push((entity, (grass.clone(), *aabb)));
    }
    commands.insert_or_spawn_batch(values);
}

pub(crate) fn extract_aabb(
    mut commands: Commands,
    aabbs: Extract<Query<(Entity, &Aabb), With<HeightMap>>>,
) {
    let mut values = Vec::new();
    for (e, aabb) in aabbs.iter() {
        values.push((e, *aabb));
    }
    commands.insert_or_spawn_batch(values);
}
