use bevy::prelude::Image;
/// The [`Eraser`] is used to erase the content of a map
///
/// This is a very destructive tool and should be used carefully
/// You can choose it in the ui of the editor
pub struct Eraser;

impl Eraser {
    pub fn erase(image: &mut Image) {
        // erase all pixels to black
        let new_image = Image::new_fill(
            image.texture_descriptor.size,
            image.texture_descriptor.dimension,
            &[0, 0, 0, 255],
            image.texture_descriptor.format,
        );
        *image = new_image;
    }
}
/// The [`Filler`] is used to fill the content of a map with the maximum value
///
/// This is a very destructive tool and should be used carefully
/// You can choose it in the ui of the editor
pub struct Filler;

impl Filler {
    // fills all pixels with white
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
