use bevy::prelude::*;

use crate::grass_spawner::GrassSpawner;

pub(crate) fn hot_reload_height_map(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut spawner_q: Query<&mut GrassSpawner>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Modified { handle } => {
                for mut spawner in spawner_q.iter_mut() {
                    if let Some(height_map) = &spawner.height_map {
                        if height_map.height_map == handle.clone() {
                            spawner.set_changed();
                            continue;
                        }
                    }
                }
                // an image was modified
            }
            _ => {}
        }
    }
}
