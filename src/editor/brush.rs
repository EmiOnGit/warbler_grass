use bevy::{prelude::*, render::render_resource::TextureFormat};

#[derive(Resource)]
pub struct ActiveBrush {
    pub brush: Box<dyn Brush>,
}
impl ActiveBrush {
    pub fn new(brush: impl Brush + 'static) -> Self {
        ActiveBrush {
            brush: Box::new(brush),
        }
    }
}
pub struct Stencil {
    size: u32,
    strength: f32,
}
impl Default for Stencil {
    fn default() -> Self {
        Self {
            size: 40,
            strength: 3.,
        }
    }
}
pub trait Brush: Sync + Send {
    /// position should be between 0 and 1
    fn draw(&mut self, image: &mut Image, position: Vec2);

    fn pixel_positions(&self, image_dimensions: Vec2, position: Vec2) -> Vec<(u32, u32)> {
        let position = (image_dimensions * position).as_ivec2();
        let range = self.size() as i32 / 2;
        (-range..range)
            .flat_map(|i| (-range..-range).map(move |j| (i, j)))
            .filter(|(x, y)| {
                position.y.checked_add(*y).is_some() && position.x.checked_add(*x).is_some()
            })
            .map(|(x, y)| (x + position.x, y + position.y))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *x >= image_dimensions.x as i32
                    && *y >= image_dimensions.y as i32
            })
            .map(|(x, y)| (x as u32, y as u32))
            .collect()
    }
    fn size(&self) -> u32;
}
impl Brush for Stencil {
    fn draw(&mut self, image: &mut Image, position: Vec2) {
        let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
            return;
        };
        let mut buffer = dynamic_image.into_rgba8();

        for (x, y) in self.pixel_positions(image.size(), position).into_iter() {
            println!("{x} {y}");
            let pixel = &mut buffer.get_pixel_mut(x as u32, y as u32).0;
            paint_gray(pixel, self.strength);
        }

        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }

    fn size(&self) -> u32 {
        self.size
    }
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
