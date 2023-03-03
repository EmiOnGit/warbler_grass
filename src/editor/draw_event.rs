

use bevy::{prelude::{Handle, Vec2, Image, Resource, EventReader, ResMut, Assets}, render::render_resource::TextureFormat};
use image::DynamicImage;


pub fn draw_map(mut active_brush: ResMut<ActiveBrush>, mut draw_events: EventReader<DrawEvent>, mut images: ResMut<Assets<Image>>) {
    for event in draw_events.iter() {
        if let DrawEvent::Draw { positions, image } = event {
            if let Some(image) = images.get_mut(image) {
                _ = active_brush.brush.draw(image, positions.clone());
            } 
        }
    }
}
#[derive(Resource)]
pub struct ActiveBrush {
    pub brush: Box<dyn Brush>,
}
impl ActiveBrush {
    pub fn new(brush: impl Brush + 'static) -> Self{
        ActiveBrush {
            brush: Box::new(brush),
        }
    }
}
pub enum DrawEvent {
    Draw {
        positions: Vec2,
        image: Handle<Image>,
    },
    Remove
}

pub trait Brush: Sync + Send {
    /// position should be between 0 and 1
    fn draw(&mut self, image: &mut Image, position: Vec2) -> Result<(), DrawError>;
}

pub struct Stencil {
    size: i32,
    strength: f32,
}
impl Default for Stencil {
    fn default() -> Self {
        Self { size: 30, strength: 1. }
    }
}
impl Brush for Stencil {
    fn draw(&mut self, image: &mut Image, position: Vec2) -> Result<(), DrawError> {
        let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
            return Err(DrawError::ImageConversionFailure);
        };
        
        let dimensions = image.size();
        let positions = (dimensions * position).as_uvec2();
        let mut buffer = dynamic_image.into_rgba8();
        for x in 0..self.size {
            for y in 0..self.size {
                let x = x - self.size / 2;
                let y = y - self.size / 2;
                let x = (positions.x as i32 - x).max(0) as u32;
                let y = (positions.y as i32 - y).max(0) as u32;
                let pixel = &mut buffer.get_pixel_mut(x,y).0;
                let s = (self.strength * 5.) as u8;
                *pixel = [pixel[0].saturating_sub(s) ,pixel[1].saturating_sub(s),pixel[2].saturating_sub(s),255];
            }
        }
        
        let dy: DynamicImage = buffer.into();
        *image = Image::from_dynamic(dy, true).convert(TextureFormat::Rgba8UnormSrgb).unwrap();
        Ok(())

    }
}
pub enum DrawError {
    ImageConversionFailure,
}
