use bevy::prelude::*;

// see https://surma.dev/things/ditherpunk/ for a good resource regarding dithering
const BAYER_DITHER: [[u8; 4]; 4] = [
    [1, 9, 3, 11],
    [13, 5, 15, 7],
    [4, 12, 2, 10],
    [16, 8, 14, 6],
];
pub fn dither_image(image: &Image) -> Option<Image> {
    let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
        return None;
    };
    let mut buffer = dynamic_image.into_luma8();
    let (width, height) = buffer.dimensions();
    for x in 0..width {
        for y in 0..height {
            let threshold = BAYER_DITHER[(x % 4) as usize][(y % 4) as usize];
            let pixel = buffer.get_pixel_mut(x, y);
            if pixel.0[0] < threshold * 10 {
                pixel.0 = [0];
            } else {
                pixel.0 = [254];
            }
        }
    }
    Some(Image::from_dynamic(buffer.into(), false))
}
