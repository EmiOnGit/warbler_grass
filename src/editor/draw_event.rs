use bevy::prelude::{Assets, EventReader, Handle, Image, ResMut, Vec2};

use super::brush::ActiveBrush;
use crate::editor::brush::Brush;

pub fn draw_map(
    mut active_brush: ResMut<ActiveBrush>,
    mut draw_events: EventReader<DrawEvent>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in draw_events.iter() {
        if let DrawEvent::Draw { positions, image } = event {
            if let Some(image) = images.get_mut(image) {
                active_brush.brush.draw(image, positions.clone());
            }
        }
    }
}
pub enum DrawEvent {
    Draw {
        positions: Vec2,
        image: Handle<Image>,
    },
    Remove,
}
