use bevy::prelude::{Assets, EventReader, Handle, Image, ResMut, Vec2};

use super::{
    brush::ActiveBrush,
    tools::{Eraser, Filler},
};

pub fn draw_map(
    mut active_brush: ResMut<ActiveBrush>,
    mut draw_events: EventReader<DrawEvent>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in draw_events.iter() {
        match event {
            DrawEvent::Draw { positions, image } => {
                if let Some(image) = images.get_mut(image) {
                    active_brush.draw(image, positions.clone());
                }
            }
            DrawEvent::Clear { image } => {
                if let Some(image) = images.get_mut(image) {
                    Eraser::erase(image);
                }
            }
            DrawEvent::Fill { image } => {
                if let Some(image) = images.get_mut(image) {
                    Filler::fill(image);
                }
            }
        }
    }
}
pub enum DrawEvent {
    Draw {
        positions: Vec2,
        image: Handle<Image>,
    },
    Clear {
        image: Handle<Image>,
    },
    Fill {
        image: Handle<Image>,
    },
}
impl DrawEvent {
    pub fn image_handle(&self) -> Option<&Handle<Image>> {
        match self {
            DrawEvent::Draw {
                positions: _,
                image,
            } => Some(&image),
            DrawEvent::Clear { image } => Some(&image),
            DrawEvent::Fill { image } => Some(&image),
        }
    }
}
