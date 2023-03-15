use super::cache::{CachedGrassChunk, EntityCache, GrassCache};
use crate::{
    bundle::{Grass, WarblerHeight},
    dithering::DitheredBuffer,
    height_map::HeightMap,
};
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
        Query<
            (
                Entity,
                &HeightMap,
                &Handle<DitheredBuffer>,
                &WarblerHeight,
                &Aabb,
            ),
            Or<(
                Changed<Handle<DitheredBuffer>>,
                Changed<HeightMap>,
                Changed<WarblerHeight>,
            )>,
        >,
    >,
    mut grass_cache: ResMut<GrassCache>,
) {
    for (entity, height_map, dithered, height, aabb) in grass_spawner.iter() {
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.dither_handle = Some(dithered.clone());
        commands.spawn((
            EntityStorage(entity),
            height_map.clone(),
            dithered.clone(),
            height.clone(),
            *aabb,
        ));
    }
}
/// Extracts the grass data of entities spawned with the [`WarblersExplicitBundle`](crate::bundle::WarblersExplicitBundle) into the render world
///
/// The extraction only happens on change or creation of the entity,
/// so it normally doesn't come at a high performance cost
#[allow(clippy::type_complexity)]
pub(crate) fn extract_grass_positions(
    mut commands: Commands,
    grass_spawner: Extract<Query<(Entity, &Grass, &Aabb), Or<(Changed<Grass>, Changed<Aabb>)>>>,
    mut grass_cache: ResMut<GrassCache>,
) {
    for (entity, grass, aabb) in grass_spawner.iter() {
        if !grass_cache.contains_key(&entity) {
            grass_cache.insert(entity, CachedGrassChunk::default());
        }
        commands.spawn((EntityStorage(entity), grass.clone(), *aabb));
    }
}

#[derive(Clone, Component)]
pub(crate) struct EntityStorage(pub Entity);

/// Extracts all visible grass entities into the render world
#[allow(clippy::type_complexity)]
pub(crate) fn extract_visibility(
    visibility_queue: Extract<
        Query<
            (Entity, &ComputedVisibility),
            (
                Or<(With<Handle<DitheredBuffer>>, With<Grass>)>,
                With<Transform>,
            ),
        >,
    >,
    mut entity_cache: ResMut<EntityCache>,
) {
    entity_cache.entities = visibility_queue
        .iter()
        .filter_map(|(e, visibility)| visibility.is_visible().then_some(e))
        .collect();
}
