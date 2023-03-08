use bevy::prelude::*;

// see https://surma.dev/things/ditherpunk/ for a good resource regarding dithering
const BAYER_DITHER: [[u8; 4]; 4] = [
    [1, 9, 3, 11],
    [13, 5, 15, 7],
    [4, 12, 2, 10],
    [16, 8, 14, 6],
];
pub fn dither_density_map(image: &Image, density: f32, field_size: Vec2) -> Option<DitheredBuffer> {
    let Ok(dynamic_image)  = image.clone().try_into_dynamic() else {
        return None;
    };
    // Capacity is not precise but should be a good estimate
    let mut dither_buffer = Vec::with_capacity(image.size().length() as usize);
    let buffer = dynamic_image.into_luma8();
    let i_count = (density * field_size.x) as usize;
    let j_count = (density * field_size.y) as usize;
    for i in 0..i_count {
        for j in 0..j_count {
            let threshold = BAYER_DITHER[i % 4][j % 4];

            //normalize i,j between 0,1
            let i = i as f32 / i_count as f32;
            let j = j as f32 / j_count as f32;
            
            let x = i * buffer.dimensions().0 as f32;            
            let y = j * buffer.dimensions().1 as f32;

            let pixel = buffer.get_pixel(x as u32, y as u32).0[0];
            if pixel > threshold {
                dither_buffer.push(Vec2::new(i * field_size.x, j * field_size.y));
            }

        }
    }
    return Some(DitheredBuffer {
        positions: dither_buffer,
    });
}

#[derive(Reflect, Clone, Component)]
pub struct DitheredBuffer {
    pub positions: Vec<Vec2>,
}
