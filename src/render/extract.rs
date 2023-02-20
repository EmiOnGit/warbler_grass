use bevy::{prelude::*, render::Extract};

use crate::prelude::Grass;

use super::cache::GrassCache;

pub(crate) fn extract_grass(
    grasses: Extract<Query<(Entity, &Grass, &GlobalTransform, &ComputedVisibility), Or<(Added<Grass>,Changed<Grass>)>>>,
    mut cache: ResMut<GrassCache>
) {
    for (entity, grass,global_transform,  comp_visibility) in grasses.iter() {
        if !comp_visibility.is_visible() {
            continue
        }
        let cache_value = cache.entry(entity).or_default();
        cache_value.transform = global_transform.clone();
        cache_value.grass = grass.clone();
    }
}