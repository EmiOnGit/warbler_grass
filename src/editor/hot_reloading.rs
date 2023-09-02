use bevy::prelude::*;

use crate::{density_map::DensityMap, prelude::WarblerHeight, y_map::YMap};

use super::draw_event::DrawEvent;

/// Sets the `DensityMap` or `HeightMap` to changed if their internal image get's changed
/// This is importend for extracting the entity again to the render world
pub fn notify_image_change(
    mut ev_asset: EventReader<DrawEvent>,
    mut q: Query<(&mut YMap, &mut DensityMap, &mut WarblerHeight)>,
) {
    for ev in ev_asset.iter() {
        let Some(image) = ev.image_handle() else {
            continue;
        };

        for (mut y_map, mut density_map, mut heights) in &mut q {
            if y_map.y_map == *image {
                y_map.as_mut();
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
