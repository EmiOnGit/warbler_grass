use bevy::asset::Asset;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages};
use bevy::render::render_resource::{Buffer, BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;

use crate::map::DensityMap;

// see https://surma.dev/things/ditherpunk/ for a good resource regarding ordered dithering
const BAYER_DITHER: [[u8; 8]; 8] = [
    [0, 32, 8, 40, 2, 34, 10, 42],
    [48, 16, 56, 24, 50, 18, 58, 26],
    [12, 44, 4, 36, 14, 46, 6, 38],
    [60, 28, 52, 20, 62, 30, 54, 22],
    [3, 35, 11, 43, 1, 33, 9, 41],
    [51, 19, 59, 27, 49, 17, 57, 25],
    [15, 47, 7, 39, 13, 45, 5, 37],
    [61, 31, 55, 23, 61, 29, 53, 21],
];
pub(crate) fn dither_density_map(
    image: &Image,
    density: f32,
    field_size: Vec2,
) -> Option<DitheredBuffer> {
    if density < 0. {
        warn!("tried to dither a image with density < 0");
        return None;
    }
    if field_size.length() < 0.0001 {
        return None;
    }
    let Ok(dynamic_image) = image.clone().try_into_dynamic() else {
        return None;
    };
    // Capacity is not precise but should be a good estimate

    let image_length = (image.size().length_squared() as f32).sqrt();
    let mut dither_buffer = Vec::with_capacity(image_length as usize);
    let buffer = dynamic_image.into_luma8();
    let i_count = (density * field_size.x).abs() as usize;
    let j_count = (density * field_size.y).abs() as usize;
    for i in 0..i_count {
        for j in 0..j_count {
            let threshold = BAYER_DITHER[i % 8][j % 8];

            //normalize i,j between 0,1
            let i = i as f32 / i_count as f32;
            let j = j as f32 / j_count as f32;

            let x = i * buffer.dimensions().0 as f32;
            let y = j * buffer.dimensions().1 as f32;

            let pixel = buffer.get_pixel(x as u32, y as u32).0[0];
            if pixel > threshold * 4 {
                dither_buffer.push(Vec2::new(i * field_size.x, j * field_size.y));
            }
        }
    }
    Some(DitheredBuffer {
        positions: dither_buffer,
    })
}
/// A buffer containing the dithered density map
///
/// This struct shouldn't be modified by the user
#[derive(Clone, Debug, TypePath, Asset)]
pub(crate) struct DitheredBuffer {
    pub positions: Vec<Vec2>,
}
/// The gpu representation of a [`DitheredBuffer`]
#[derive(Debug)]
pub(crate) struct GpuDitheredBuffer {
    pub buffer: Buffer,
    pub instances: usize,
}
impl RenderAsset for DitheredBuffer {
    type PreparedAsset = GpuDitheredBuffer;

    type Param = SRes<RenderDevice>;

    fn asset_usage(&self) -> bevy::render::render_asset::RenderAssetUsages {
        RenderAssetUsages::default()
    }

    fn prepare_asset(
        self,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
        let render_device = param;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: "dither buffer".into(),
            contents: bytemuck::cast_slice(self.positions.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        Ok(GpuDitheredBuffer {
            buffer,
            instances: self.positions.len(),
        })
    }
}

/// Updates the [`DitheredBuffer`] of an entity
#[allow(clippy::type_complexity)]
pub(crate) fn add_dither_to_density(
    mut commands: Commands,
    grasses: Query<(Entity, &DensityMap, &Aabb), Or<(Changed<DensityMap>, Changed<Aabb>)>>,
    images: Res<Assets<Image>>,
    mut dithered: ResMut<Assets<DitheredBuffer>>,
    mut storage: Local<Vec<(Entity, DensityMap, Aabb)>>,
) {
    let stored = std::mem::take(&mut *storage);
    for (e, density_map, aabb) in grasses
        .iter()
        .chain(stored.iter().map(|(e, map, aabb)| (*e, map, aabb)))
    {
        if let Some(image) = images.get(&density_map.density_map) {
            let xz = aabb.half_extents.xz() * 2.;
            let Some(buffer) = dither_density_map(image, density_map.density, xz) else {
                warn!("Couldn't dither density map. Maybe the image format is not supported?");
                continue;
            };
            let handle = dithered.add(buffer);
            commands.entity(e).insert(handle);
        } else {
            storage.push((e, density_map.clone(), *aabb));
        }
    }
}
#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::prelude::Image;
    use bevy::render::render_asset::RenderAssetUsages;
    #[test]
    fn dither_1x1() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(&image, 1., Vec2::new(1., 1.));
        assert!(dither.is_some());
        assert_eq!(dither.unwrap().positions.len(), 1);
        let dither = super::dither_density_map(&image, 1., Vec2::new(10., 5.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.len() == 10 * 5);
    }
    #[test]
    fn dither_density() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(&image, 2., Vec2::new(1., 1.));
        assert_eq!(dither.unwrap().positions.len(), 2 * 2);
        let dither = super::dither_density_map(&image, 2., Vec2::new(10., 5.));
        assert!(dither.unwrap().positions.len() == (10 * 2) * (5 * 2));
        let dither = super::dither_density_map(&image, 5., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.len() == 5 * 5);
        let dither = super::dither_density_map(&image, 0.1, Vec2::new(10., 10.));
        assert!(dither.unwrap().positions.len() == 1);

        // transform the image to be black
        let dynamic = image.try_into_dynamic().unwrap();
        let mut luma = dynamic.to_luma8();
        let pixel = luma.get_pixel_mut(0, 0);
        pixel.0 = [0];
        // this image is now black
        let image = Image::from_dynamic(luma.into(), true, RenderAssetUsages::empty());
        // with a black image we expect 0 grassblades regardless of density
        let dither = super::dither_density_map(&image, 2., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 20., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 2., Vec2::new(10., 5.));
        assert!(dither.unwrap().positions.is_empty());
    }
    #[test]
    fn wrong_input() {
        let image = Image::default(); // 1x1x1 image all white
                                      // density=0 should return 0 results but still work
        let dither = super::dither_density_map(&image, 0., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        // negative density should return None
        let dither = super::dither_density_map(&image, -1., Vec2::new(1., 1.));
        assert!(dither.is_none());
        let dither = super::dither_density_map(&image, 1., Vec2::new(0., 0.));
        assert!(dither.is_none());
    }
    #[test]
    fn dither_field_size() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(&image, 1., Vec2::new(10., 1.));
        assert!(dither.is_some());
        let dither = super::dither_density_map(&image, 1., Vec2::new(10., 10.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.len() == 10 * 10);
        let dither = super::dither_density_map(&image, 1., Vec2::new(0., 10.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 1., Vec2::new(100., 0.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 1., Vec2::new(-10., 0.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 1., Vec2::new(0., -10.));
        assert!(dither.is_some());
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(&image, 1., Vec2::new(-10., -10.));
        assert!(dither.is_some());
        assert_eq!(dither.unwrap().positions.len(), 100);
        let dither = super::dither_density_map(&image, 1., Vec2::new(-10., 5.));
        assert!(dither.is_some());
        assert_eq!(dither.unwrap().positions.len(), 50);
    }
}
