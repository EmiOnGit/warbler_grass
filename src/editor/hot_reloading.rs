use bevy::prelude::*;

use crate::{density_map::DensityMap, height_map::HeightMap, prelude::WarblerHeight};

use super::draw_event::DrawEvent;

/// Sets the `DensityMap` or `HeightMap` to changed if their internal image get's changed
/// This is importend for extracting the entity again to the render world
pub fn notify_image_change(
    mut ev_asset: EventReader<DrawEvent>,
    mut q: Query<(&mut HeightMap, &mut DensityMap, &mut WarblerHeight)>,
) {
    for ev in ev_asset.iter() {
        let Some(image) = ev.image_handle() else {
            continue;
        };

        for (mut height_map, mut density_map, mut heights) in &mut q {
            if height_map.height_map == *image {
                height_map.as_mut();
            }
            if density_map.density_map == *image {
                density_map.as_mut();
            }
            if let WarblerHeight::Texture(texture) = heights.as_ref() {
                if texture == image {
                    heights.as_mut();
                }
            }
        }
    }
}
