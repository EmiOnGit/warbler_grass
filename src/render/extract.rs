use super::cache::{EntityCache, GrassCache};
use crate::{dithering::DitheredBuffer, grass_spawner::GrassSpawner};
use bevy::{
    prelude::*,
    render::{primitives::Aabb, Extract},
};

/// Extracts the grass data into the render world.
///
/// The extraction only happens on change or creation of the entity,
/// so it normally doesn't come at a high performance cost.
///
/// Note:
/// 1) Currently, the grass data extracted in the render world doesn't get freed when the grass entity is deleted.
/// 2) If you are changing your grass data constantly you might run into performance problems rather quickly
#[allow(clippy::type_complexity)]
pub(crate) fn extract_grass(
    mut commands: Commands,
    grass_spawner: Extract<
        Query<
            (Entity, &GrassSpawner, &GlobalTransform, &Aabb),
            Or<(Changed<GrassSpawner>, Changed<Aabb>)>,
        >,
    >,
    mut grass_cache: ResMut<GrassCache>,
) {
    for (entity, spawner, global_transform, aabb) in grass_spawner.iter() {
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.transform = *global_transform;
        commands
            .spawn(spawner.clone())
            .insert(EntityStorage(entity))
            .insert(*aabb);
    }
}
#[derive(Clone, Component)]
pub(crate) struct EntityStorage(pub Entity);
/// Extracts all visible grass entities into the render world.
pub(crate) fn extract_visibility(
    visibility_queue: Extract<
        Query<(Entity, &ComputedVisibility), (With<GrassSpawner>, With<Transform>)>,
    >,
    mut entity_cache: ResMut<EntityCache>,
) {
    entity_cache.entities = visibility_queue
        .iter()
        .filter_map(|(e, visibility)| visibility.is_visible().then_some(e))
        .collect();
}
pub(crate) fn extract_dither_map(
    spawner: Extract<Query<(Entity, &DitheredBuffer), Changed<DitheredBuffer>>>,
    mut commands: Commands,
) {
    spawner.iter().for_each(|(e, dither_buffer)| {
        commands
            .spawn(EntityStorage(e))
            .insert(dither_buffer.clone());
    });
}