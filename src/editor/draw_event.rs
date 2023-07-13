use bevy::prelude::{Assets, Event, EventReader, Handle, Image, ResMut, Vec2};

use super::{
    tools::{Eraser, Filler},
    ui::ActiveTool,
};

pub fn draw_map(
    mut active_brush: ResMut<ActiveTool>,
    mut draw_events: EventReader<DrawEvent>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in draw_events.iter() {
        match event {
            DrawEvent::Draw { positions, image } => {
                if let Some(image) = images.get_mut(image) {
                    active_brush.apply(image, *positions);
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

#[derive(Event)]
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
            } => Some(image),
            DrawEvent::Clear { image } => Some(image),
            DrawEvent::Fill { image } => Some(image),
        }
    }
}
