use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::Vec3Swizzles,
    prelude::*,
    reflect::TypeUuid,
    render::{
        primitives::Aabb,
        render_asset::{RenderAsset, PrepareAssetError},
        render_resource::{Buffer, BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
    },
};
use serde::Deserialize;

use crate::density_map::DensityMap;

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
            if pixel > threshold * 15 {
                dither_buffer.push(Vec2::new(i * field_size.x, j * field_size.y));
            }
        }
    }
    Some(DitheredBuffer {
        positions: dither_buffer,
    })
}

#[derive(Reflect, Clone, Component, Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct DitheredBuffer {
    pub positions: Vec<Vec2>,
}
pub struct GpuDitheredBuffer {
    pub buffer: Buffer,
    pub instances: usize,
}
impl RenderAsset for DitheredBuffer {
    type ExtractedAsset = DitheredBuffer;

    type PreparedAsset = GpuDitheredBuffer;

    type Param = SRes<RenderDevice>;

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        PrepareAssetError<Self::ExtractedAsset>,
    > {
        let render_device = param;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: "dither buffer".into(),
            contents: bytemuck::cast_slice(extracted_asset.positions.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        Ok(GpuDitheredBuffer {
            buffer,
            instances: extracted_asset.positions.len(),
        })
    }
}
#[allow(clippy::type_complexity)]
pub(crate) fn add_dither_to_density(
    mut commands: Commands,
    grasses: Query<(Entity, &DensityMap, &Aabb), Or<(Changed<DensityMap>, Changed<Aabb>)>>,
    images: Res<Assets<Image>>,
    mut dithered: ResMut<Assets<DitheredBuffer>>,
) {
    for (e, density_map, aabb) in grasses.iter() {
        if let Some(image) = images.get(&density_map.density_map) {
            let xz = aabb.half_extents.xz() * 2.;
            let Some(buffer) = dither_density_map(image, density_map.density, xz) else {
                warn!("couldn't dither density map");
                continue
            };
            let handle = dithered.add(buffer);
            commands.entity(e).insert(handle);
        }
    }
}
