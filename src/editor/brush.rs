use bevy::{prelude::*, render::render_resource::TextureFormat};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct ActiveBrush {
    pub brush: Brushes,
    brush_size: u32,
    strength: f32,
}

impl ActiveBrush {
    pub fn new(brush: Brushes) -> Self {
        ActiveBrush {
            brush,
            brush_size: 5,
            strength: 10.,
        }
    }
    pub fn draw(&mut self, image: &mut Image, position: Vec2) {
        self.brush
            .draw(image, position, self.brush_size, self.strength);
    }
}
#[derive(Reflect, FromReflect, InspectorOptions)]

#[reflect(InspectorOptions)]
pub enum Brushes {
    Stencil,
    Airbrush,
}
impl Default for Brushes {
    fn default() -> Self {
        Self::Airbrush
    }
}

impl Brushes {
    fn draw(&self, image: &mut Image, position: Vec2, brush_size: u32, strength: f32) {
        match self {
            Self::Stencil => Stencil::draw(image, position, brush_size, strength),
            Self::Airbrush=> Airbrush::draw(image, position, brush_size, strength),
        }
    }
}

pub trait Brush: Sync + Send {
    /// position should be between 0 and 1
    fn draw(image: &mut Image, position: Vec2, brush_size: u32, strength: f32);
}
#[derive(Reflect, FromReflect, Default)]
pub struct Stencil;

impl Brush for Stencil {
    fn draw(image: &mut Image, position: Vec2, brush_size: u32, strength: f32) {
        let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
            warn!("couldn't convert image");
            return;
        };
        let mut buffer = dynamic_image.into_rgba8();

        for (x, y) in pixel_positions(brush_size, image.size(), position).into_iter() {
            let pixel = &mut buffer.get_pixel_mut(x as u32, y as u32).0;
            paint_gray(pixel, strength);
        }

        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }
}

#[derive(Reflect, FromReflect, Default, InspectorOptions)]

#[reflect(InspectorOptions)] 
pub struct Airbrush;
impl Brush for Airbrush {
    fn draw(image: &mut Image, position: Vec2, brush_size: u32, strength: f32) {
        let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
            warn!("couldn't convert image");
            return;
        };
        let mut buffer = dynamic_image.into_rgba8();
        let positions = pixel_positions(brush_size, image.size(), position);
        let mut max = (u32::MIN,u32::MIN);
        let mut center = positions
            .iter()
            .map(|(x,y)| {
                if x >= &max.0  && y >= &max.1 {
                    max = (*x,*y);
                }
              
                (x,y)
            })
            .fold((0, 0), |(sumx, sumy), (x, y)| (sumx + x, sumy + y));
        

        center = (center.0 / positions.len() as u32, center.1 / positions.len() as u32);
        let max_distance = (max.0 as f32 - center.0 as f32).powf(2.) + (max.1 as f32 - center.1 as f32).powf(2.);

        for (x, y) in positions.into_iter() {
            let pixel = &mut buffer.get_pixel_mut(x as u32, y as u32).0;
            
            let distance = ((((x as f32 - center.0 as f32)).powf(2.) + ((y as f32 - center.1 as f32)).powf(2.)) / max_distance).powf(0.1) ;
            let total_strength = strength - (strength * distance);
            
            paint_gray(pixel, total_strength);
        }

        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }
}

fn pixel_positions(brush_size: u32, image_dimensions: Vec2, position: Vec2) -> Vec<(u32, u32)> {
    let position = (image_dimensions * position).as_ivec2();
    let range = brush_size as i32 * (image_dimensions.x + image_dimensions.y) as i32 / 100;
    (-range..range)
        .flat_map(|i| (-range..range).map(move |j| (i, j)))
        .filter(|(x, y)| {
            position.y.checked_add(*y).is_some() && position.x.checked_add(*x).is_some()
        })
        .map(|(x, y)| (x + position.x, y + position.y))
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && *x < image_dimensions.x as i32 && *y < image_dimensions.y as i32
        })
        .map(|(x, y)| (x as u32, y as u32))
        .collect()
}
fn paint_gray(pixel: &mut [u8; 4], strength: f32) {
    if strength >= 0. {
        let strength = strength as u8;
        *pixel = [
            pixel[0].saturating_add(strength),
            pixel[1].saturating_add(strength),
            pixel[2].saturating_add(strength),
            255,
        ];
    } else {
        let strength = -strength as u8;
        *pixel = [
            pixel[0].saturating_sub(strength),
            pixel[1].saturating_sub(strength),
            pixel[2].saturating_sub(strength),
            255,
        ];
    }
}
