use bevy::prelude::*;

use crate::{density_map::DensityMap, height_map::HeightMap, prelude::WarblerHeight};

use super::draw_event::DrawEvent;

// use crate::grass_spawner::GrassSpawner;
/// Sets the `DensityMap` or `HeightMap` to changed if their internal image get's changed
/// This is importend for extracting the entity again to the render world
pub fn notify_image_change(
    mut ev_asset: EventReader<DrawEvent>,
    mut q: Query<
        (
            Option<&mut HeightMap>,
            Option<&mut DensityMap>,
            &mut WarblerHeight,
        ),
        Or<(With<HeightMap>, With<DensityMap>)>,
    >,
) {
    for ev in ev_asset.iter() {
        let Some(image) = ev.image_handle() else {
            continue;
        };

        for (height_map, density_map, mut heights) in &mut q {
            if let Some(height_map_ref) = height_map.as_ref() {
                if height_map_ref.height_map == *image {
                    height_map.unwrap().as_mut();
                }
            }
            if let Some(density_map_ref) = density_map.as_ref() {
                if density_map_ref.density_map == *image {
                    density_map.unwrap().as_mut();
                }
            }
            if let WarblerHeight::Texture(texture) = heights.as_ref() {
                if texture == image {
                    heights.as_mut();
                }
            }
        }
    }
}
