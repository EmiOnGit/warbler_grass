use bevy::prelude::*;

use crate::{density_map::DensityMap, height_map::HeightMap};

use super::draw_event::DrawEvent;

// use crate::grass_spawner::GrassSpawner;
/// Sets the `DensityMap` or `HeightMap` to changed if their internal image get's changed
/// This is importend for extracting the entity again to the render world
pub fn notify_image_change(
    mut ev_asset: EventReader<DrawEvent>,
    mut q: Query<
        (Option<&mut HeightMap>, Option<&mut DensityMap>),
        Or<(With<HeightMap>, With<DensityMap>)>,
    >,
) {
    for ev in ev_asset.iter() {
        if let DrawEvent::Draw {
            positions: _,
            image: handle,
        } = ev
        {
            for (height_map, density_map) in &mut q {
                if let Some(height_map_ref) = height_map.as_ref() {
                    if height_map_ref.height_map == *handle {
                        height_map.unwrap().as_mut();
                    }
                }
                if let Some(density_map_ref) = density_map.as_ref() {
                    if density_map_ref.density_map == *handle {
                        density_map.unwrap().as_mut();
                    }
                }
            }
        }
    }
}
