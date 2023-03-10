use bevy::prelude::Image;

pub struct Eraser;

impl Eraser {
    pub fn erase(image: &mut Image) {
        let new_image = Image::new_fill(
            image.texture_descriptor.size,
            image.texture_descriptor.dimension,
            &[0, 0, 0, 255],
            image.texture_descriptor.format,
        );
        *image = new_image;
    }
}

pub struct Filler;

impl Filler {
    pub fn fill(image: &mut Image) {
        let new_image = Image::new_fill(
            image.texture_descriptor.size,
            image.texture_descriptor.dimension,
            &[255, 255, 255, 255],
            image.texture_descriptor.format,
        );
        *image = new_image;
    }
}
