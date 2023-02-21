use bevy::{prelude::*, render::Extract};

use crate::prelude::Grass;

use super::cache::{EntityCache, GrassCache};

#[allow(clippy::type_complexity)]
pub(crate) fn extract_grass(
    grasses: Extract<
        Query<
            (Entity, &Grass, &GlobalTransform, &ComputedVisibility),
            Or<(Added<Grass>, Changed<Grass>)>,
        >,
    >,
    mut grass_cache: ResMut<GrassCache>,
    mut entity_cache: ResMut<EntityCache>,
) {
    for (entity, grass, global_transform, comp_visibility) in grasses.iter() {
        if !comp_visibility.is_visible() {
            continue;
        }
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.transform = *global_transform;
        cache_value.grass = grass.clone();
        if !entity_cache.contains(&entity) {
            entity_cache.push(entity);
        }
    }
}
