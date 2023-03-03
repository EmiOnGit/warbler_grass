use bevy::{
    prelude::{Assets, EventReader, Handle, Image, ResMut, Resource, Vec2},
    render::render_resource::TextureFormat,
};

pub fn draw_map(
    mut active_brush: ResMut<ActiveBrush>,
    mut draw_events: EventReader<DrawEvent>,
    mut images: ResMut<Assets<Image>>,
) {
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
    pub fn new(brush: impl Brush + 'static) -> Self {
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
    Remove,
}

pub trait Brush: Sync + Send {
    /// position should be between 0 and 1
    fn draw(&mut self, image: &mut Image, position: Vec2);
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
impl Brush for Stencil {
    fn draw(&mut self, image: &mut Image, position: Vec2) {
        let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
            return;
        };

        let dimensions = image.size();
        let position = (dimensions * position).as_ivec2();
        let mut buffer = dynamic_image.into_rgba8();
        let range = self.size as i32 / 2;
        for x in -range..range {
            for y in -range..range {
                if position.x.checked_add(x).is_none() || position.y.checked_add(y).is_none() {
                    continue;
                }

                let Some(x) = position.x.checked_add(x) else {
                    continue;
                };
                let Some(y) = position.y.checked_add(y) else {
                    continue;
                };
                if x < 0 || y < 0 {
                    continue;
                }
                if x >= dimensions.x as i32 {
                    continue;
                }
                if y >= dimensions.y as i32 {
                    continue;
                }
                let pixel = &mut buffer.get_pixel_mut(x as u32, y as u32).0;

                let strength = self.strength as u8;
                *pixel = [
                    pixel[0].saturating_add(strength),
                    pixel[1].saturating_add(strength),
                    pixel[2].saturating_add(strength),
                    255,
                ];
            }
        }
        *image = Image::from_dynamic(buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .unwrap();
    }
}
