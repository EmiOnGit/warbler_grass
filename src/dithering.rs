use std::error::Error;
use std::fmt::Display;

use bevy::asset::Asset;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::{CommandQueue, SystemParamItem};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages};
use bevy::render::render_resource::{Buffer, BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use image::DynamicImage;

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
const MIN_AREA: f32 = 0.0001;
#[derive(PartialEq, Debug)]
pub enum DitherComputeError {
    ImageFormat,
    /// The density is too small to be dithered.
    /// Usually this means that the provided density is negative.
    /// The provided density is stored in the error
    DensityToSmall(f32),
    /// The area is too small to be dithered.
    /// The error contains the area of the chunk.
    /// The area is calculated using `field_size.x * field_size.y`
    ChunkAreaToSmall(f32),
}
impl Display for DitherComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DitherComputeError::ImageFormat => write!(f, "The densitymap was not in a supported `ImageFormat`. The recommended format is Luma8(R8), but it should be at least convertable to Luma8"),
            DitherComputeError::DensityToSmall(density) => write!(f, "The density has to be larger than 0, but was {density}"),
            DitherComputeError::ChunkAreaToSmall(area) => write!(f, "The chunk area is to tiny. Current area is {area} but has to be at least {MIN_AREA}"),
        }
    }
}
impl Error for DitherComputeError {}
/// Dithers a given density map.
/// The performance is highly dependend on the image type of the density map. If the image is already encoded in luma8 (or r8) format,
/// the dithering is substancially faster.
pub(crate) fn dither_density_map(
    image: Image,
    density: f32,
    field_size: Vec2,
) -> Result<DitheredBuffer, DitherComputeError> {
    if density < 0. {
        return Err(DitherComputeError::DensityToSmall(density));
    }

    let area = field_size.x * field_size.y;
    if area < MIN_AREA {
        return Err(DitherComputeError::ChunkAreaToSmall(area));
    }
    let image_length = (image.size().length_squared() as f32).sqrt();
    let Ok(dynamic_image) = image.try_into_dynamic() else {
        return Err(DitherComputeError::ImageFormat);
    };
    // Capacity is not precise but should be a good estimate

    let mut dither_buffer = Vec::with_capacity(image_length as usize);
    if !matches!(dynamic_image, DynamicImage::ImageLuma8(_)) {
        warn_once!("The density map is prefered to be in Luma8(/R8) encoding");
    }
    // This conversion doesn't cost anything if the image is already luma8
    // but makes up for most of the function duration otherwise.
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
    Ok(DitheredBuffer {
        positions: dither_buffer,
    })
}
#[derive(Component)]
pub(crate) struct ComputeDither(Task<CommandQueue>);
/// A buffer containing the dithered density map
///
/// This struct shouldn't be modified by the user
#[derive(Clone, Debug, TypePath, Asset, PartialEq)]
pub(crate) struct DitheredBuffer {
    pub positions: Vec<Vec2>,
}
/// The gpu representation of a [`DitheredBuffer`]
#[derive(Debug)]
pub(crate) struct GpuDitheredBuffer {
    pub buffer: Buffer,
    pub instances: usize,
}
impl RenderAsset for GpuDitheredBuffer {
    type SourceAsset = DitheredBuffer;

    type Param = SRes<RenderDevice>;

    fn prepare_asset(
        source_asset: Self::SourceAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>> {
        let render_device = param;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: "dither buffer".into(),
            contents: bytemuck::cast_slice(source_asset.positions.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        Ok(GpuDitheredBuffer {
            buffer,
            instances: source_asset.positions.len(),
        })
    }
}
pub(crate) fn add_dither_task(
    mut commands: Commands,
    grasses: Query<(Entity, &DensityMap, &Aabb), Or<(Changed<DensityMap>, Changed<Aabb>)>>,
    images: Res<Assets<Image>>,
    mut storage: Local<Vec<(Entity, DensityMap, Aabb)>>,
    mut event_writer: EventWriter<GrassComputeEvent>,
) {
    if storage.is_empty() && grasses.is_empty() {
        return;
    }
    let stored = std::mem::take(&mut *storage);
    let thread_pool: &AsyncComputeTaskPool = AsyncComputeTaskPool::get();
    let mut data = Vec::new();
    for (e, density_map, aabb) in grasses
        .iter()
        .chain(stored.iter().map(|(e, map, aabb)| (*e, map, aabb)))
    {
        let Some(image) = images.get(&density_map.density_map) else {
            // the entity might have been deleted from the world
            if commands.get_entity(e).is_some() {
                storage.push((e, density_map.clone(), *aabb));
            }
            continue;
        };
        data.push((e, image.clone(), density_map.density, *aabb));
    }
    for (e, map, density, aabb) in data.into_iter() {
        event_writer.send(GrassComputeEvent::StartComputation(e));
        let task: Task<_> = thread_pool.spawn::<CommandQueue>(async move {
            let mut command_queue = CommandQueue::default();
            let xz = aabb.half_extents.xz() * 2.;
            // We want to remove `ComputeDither` regardless of success to avoid polling on a already finished task
            // (Which would crash the app)
            command_queue.push(move |world: &mut World| {
                if let Some(mut entity_builder) = world.get_entity_mut(e) {
                    entity_builder.remove::<ComputeDither>();
                }
            });
            match dither_density_map(map, density, xz) {
                Ok(buffer) => {
                    command_queue.push(move |world: &mut World| {
                        let event = on_dither_success(world, e, buffer);
                        world.send_event(event);
                    });
                }
                Err(error) => {
                    command_queue.push(move |world: &mut World| {
                        world.send_event::<GrassComputeEvent>(
                            GrassComputeError::FailedComputation(e, error).into(),
                        );
                    });
                }
            }
            command_queue
        });
        commands.entity(e).try_insert(ComputeDither(task));
    }
}
fn on_dither_success(world: &mut World, e: Entity, buffer: DitheredBuffer) -> GrassComputeEvent {
    let Some(mut dithered) = world.get_resource_mut::<Assets<DitheredBuffer>>() else {
        return GrassComputeError::FailedRequestResource.into();
    };
    let handle = dithered.add(buffer);
    if let Some(mut entity_builder) = world.get_entity_mut(e) {
        entity_builder.insert(handle);
        return GrassComputeEvent::FinishedComputation(e);
    } else {
        return GrassComputeError::EntityDoesNotExist(e).into();
    }
}

pub(crate) fn check_dither_compute_tasks(
    mut commands: Commands,
    mut dither_tasks: Query<&mut ComputeDither>,
) {
    for mut task in &mut dither_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }
}
#[derive(Event)]
pub enum GrassComputeEvent {
    StartComputation(Entity),
    FinishedComputation(Entity),
    Error(GrassComputeError),
}
impl From<GrassComputeError> for GrassComputeEvent {
    fn from(value: GrassComputeError) -> Self {
        GrassComputeEvent::Error(value)
    }
}
#[derive(Debug)]
pub enum GrassComputeError {
    FailedComputation(Entity, DitherComputeError),
    FailedRequestResource,
    EntityDoesNotExist(Entity),
}
impl Display for GrassComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrassComputeError::FailedComputation(_, er) => er.fmt(f),
            GrassComputeError::FailedRequestResource => write!(f,"Failed to request the `DitherBuffer` assets, which should be inserted with the `WarblerPlugin` at the start of the app"),
            GrassComputeError::EntityDoesNotExist(e) => write!(f,"Entity {e:?} does not exist anymore. Maybe the entity was removed while calculating the grass positions?"),
        }
    }
}
impl Error for GrassComputeError {}
#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::prelude::Image;
    use bevy::render::render_asset::RenderAssetUsages;

    use crate::dithering::DitherComputeError;
    #[test]
    fn dither_1x1() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(1., 1.));
        assert!(dither.is_ok());
        assert_eq!(dither.unwrap().positions.len(), 1);
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(10., 5.));
        assert!(dither.is_ok());
        assert!(dither.unwrap().positions.len() == 10 * 5);
    }
    #[test]
    fn dither_density() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(image.clone(), 2., Vec2::new(1., 1.));
        assert_eq!(dither.unwrap().positions.len(), 2 * 2);
        let dither = super::dither_density_map(image.clone(), 2., Vec2::new(10., 5.));
        assert!(dither.unwrap().positions.len() == (10 * 2) * (5 * 2));
        let dither = super::dither_density_map(image.clone(), 5., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.len() == 5 * 5);
        let dither = super::dither_density_map(image.clone(), 0.1, Vec2::new(10., 10.));
        assert!(dither.unwrap().positions.len() == 1);

        // transform the image to be black
        let dynamic = image.try_into_dynamic().unwrap();
        let mut luma = dynamic.to_luma8();
        let pixel = luma.get_pixel_mut(0, 0);
        pixel.0 = [0];
        // this image is now black
        let image = Image::from_dynamic(luma.into(), true, RenderAssetUsages::empty());
        // with a black image we expect 0 grassblades regardless of density
        let dither = super::dither_density_map(image.clone(), 2., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(image.clone(), 20., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        let dither = super::dither_density_map(image.clone(), 2., Vec2::new(10., 5.));
        assert!(dither.unwrap().positions.is_empty());
    }
    #[test]
    fn wrong_input() {
        let image = Image::default(); // 1x1x1 image all white
                                      // density=0 should return 0 results but still work
        let dither = super::dither_density_map(image.clone(), 0., Vec2::new(1., 1.));
        assert!(dither.unwrap().positions.is_empty());
        // negative density should return None
        let dither = super::dither_density_map(image.clone(), -1., Vec2::new(1., 1.));
        assert!(dither.is_err());
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(0., 0.));
        assert!(dither.is_err());
    }
    #[test]
    fn dither_field_size() {
        let image = Image::default(); // 1x1x1 image all white
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(10., 1.));
        assert!(dither.is_ok());
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(10., 10.));
        assert!(dither.is_ok());
        assert!(dither.unwrap().positions.len() == 10 * 10);
        let dither = super::dither_density_map(image.clone(), 0., Vec2::new(10., 10.));
        assert!(dither.is_ok());
        assert!(dither.unwrap().positions.is_empty());

        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(0., 10.));
        assert_eq!(dither, Err(DitherComputeError::ChunkAreaToSmall(0.)));
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(100., 0.));
        assert_eq!(dither, Err(DitherComputeError::ChunkAreaToSmall(0.)));
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(0., 0.));
        assert_eq!(dither, Err(DitherComputeError::ChunkAreaToSmall(0.)));
        let dither = super::dither_density_map(image.clone(), 1., Vec2::new(-10., 0.));
        assert_eq!(dither, Err(DitherComputeError::ChunkAreaToSmall(0.)));

        let dither = super::dither_density_map(image.clone(), -0.1, Vec2::new(10., 10.));
        assert_eq!(dither, Err(DitherComputeError::DensityToSmall(-0.1)));
    }
}
