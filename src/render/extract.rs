use super::cache::{EntityCache, GrassCache};
use crate::prelude::Grass;
use bevy::{prelude::*, render::Extract};

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
    grasses: Extract<Query<(Entity, &Grass, &GlobalTransform), Changed<Grass>>>,
    mut grass_cache: ResMut<GrassCache>,
) {
    for (entity, grass, global_transform) in grasses.iter() {
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.transform = *global_transform;
        cache_value.grass = grass.clone();
        commands.spawn(grass.clone()).insert(EntityStore(entity));
    }
}
#[derive(Clone, Component)]
pub(crate) struct EntityStore(pub Entity);
/// Extracts all visible grass entities into the render world.
pub(crate) fn extract_visibility(
    visibility_queue: Extract<Query<(Entity, &ComputedVisibility), (With<Grass>, With<Transform>)>>,
    mut entity_cache: ResMut<EntityCache>,
) {
    entity_cache.entities = visibility_queue
        .iter()
        .filter_map(|(e, visibility)| visibility.is_visible().then(|| e))
        .collect();
}
