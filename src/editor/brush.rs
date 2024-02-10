use bevy::{prelude::*, render::render_resource::TextureFormat};

pub trait BrushBehavior: Sync + Send {
    /// position should be between 0 and 1
    fn draw(&self, image: &mut Image, position: Vec2, brush_size: u32, strength: f32);
}
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct Stencil;

impl BrushBehavior for Stencil {
    fn draw(&self, image: &mut Image, position: Vec2, brush_size: u32, strength: f32) {
        let Ok(dynamic_image) = image.clone().try_into_dynamic() else {
            warn!("couldn't convert image");
            return;
        };
        let mut buffer = dynamic_image.into_rgba8();

        for (x, y) in pixel_positions(brush_size, image.size(), position).into_iter() {
            let pixel = &mut buffer.get_pixel_mut(x, y).0;
            paint_gray(pixel, strength);
        }

        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }
}

#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct Airbrush;
impl BrushBehavior for Airbrush {
    fn draw(&self, image: &mut Image, position: Vec2, brush_size: u32, strength: f32) {
        let Ok(dynamic_image) = image.clone().try_into_dynamic() else {
            warn!("couldn't convert image");
            return;
        };
        let mut buffer = dynamic_image.into_rgba8();
        let positions = pixel_positions(brush_size, image.size(), position);
        let mut max = (u32::MIN, u32::MIN);
        let mut center = positions
            .iter()
            .map(|(x, y)| {
                if x >= &max.0 && y >= &max.1 {
                    max = (*x, *y);
                }

                (x, y)
            })
            .fold((0, 0), |(sumx, sumy), (x, y)| (sumx + x, sumy + y));

        center = (
            center.0 / positions.len() as u32,
            center.1 / positions.len() as u32,
        );
        let max_distance =
            (max.0 as f32 - center.0 as f32).powf(2.) + (max.1 as f32 - center.1 as f32).powf(2.);

        for (x, y) in positions.into_iter() {
            let pixel = &mut buffer.get_pixel_mut(x, y).0;

            let distance = (((x as f32 - center.0 as f32).powf(2.)
                + (y as f32 - center.1 as f32).powf(2.))
                / max_distance)
                .powf(0.1);
            let total_strength = strength - (strength * distance);

            paint_gray(pixel, total_strength);
        }

        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }
}

fn pixel_positions(brush_size: u32, image_dimensions: UVec2, position: Vec2) -> Vec<(u32, u32)> {
    let position = (image_dimensions.as_vec2() * position).as_ivec2();
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
