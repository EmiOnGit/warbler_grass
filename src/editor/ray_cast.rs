use bevy::prelude::*;
use bevy::{math::Vec3Swizzles, render::primitives::Aabb};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{
    density_map::DensityMap,
    prelude::{WarblerHeight, YMap},
};

use super::{draw_event::DrawEvent, ActiveEditorChunk};
pub(super) struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_collision_on_click, update_camera_ray));
    }
}
/// Indicates the currently selected map in the editor
/// can be directly choosen from the ui
#[derive(Resource, Reflect, Default, InspectorOptions, PartialEq)]
#[reflect(Resource, InspectorOptions)]
pub enum SelectedMap {
    YMap,
    #[default]
    DensityMap,
    HeightsMap,
}
#[allow(clippy::type_complexity)]
fn check_collision_on_click(
    mut active_chunk: ResMut<ActiveEditorChunk>,
    grass_chunk: Query<
        (
            Entity,
            &Transform,
            &Aabb,
            &DensityMap,
            &YMap,
            &WarblerHeight,
        ),
        Without<RayCamera>,
    >,
    camera_source: Query<&RayCamera>,
    mouse_presses: Res<Input<MouseButton>>,
    selection: Res<SelectedMap>,
    mut draw_events: EventWriter<DrawEvent>,
) {
    if !mouse_presses.pressed(MouseButton::Left) {
        return;
    }
    let Ok(raycast_camera) = camera_source.get_single() else {
        return;
    };
    let click_ray = raycast_camera.ray.as_ref().unwrap();
    for (entity, chunk_transform, aabb, density_map, y_map, heights) in &grass_chunk {
        let aabb_center = aabb.center.as_dvec3().as_vec3() + chunk_transform.translation;

        let Some(intersection_distance) = click_ray.intersect_plane(aabb_center, Vec3::Y) else {
            continue;
        };
        let res = click_ray.get_point(intersection_distance);

        let intersection_point = (res - aabb_center).xz();
        let aabb_extends = aabb.half_extents.as_dvec3().as_vec3().xz().abs();
        if aabb_extends.x > intersection_point.x
            && -aabb_extends.x < intersection_point.x
            && aabb_extends.y > intersection_point.y
            && -aabb_extends.y < intersection_point.y
        {
            let positions = (Vec2::new(
                intersection_point.x / aabb_extends.x,
                intersection_point.y / aabb_extends.y,
            ) + Vec2::ONE)
                / 2.;
            let image = match *selection {
                SelectedMap::YMap => y_map.y_map.clone(),
                SelectedMap::DensityMap => density_map.density_map.clone(),
                SelectedMap::HeightsMap => {
                    if let WarblerHeight::Texture(image) = heights {
                        image.clone()
                    } else {
                        warn!("No heights texture found. Using density map instead");
                        density_map.density_map.clone()
                    }
                }
            };
            active_chunk.0 = Some(entity);
            if mouse_presses.pressed(MouseButton::Left) {
                draw_events.send(DrawEvent::Draw { positions, image });
            }
        }
    }
}
/// Indicates a camera which can raycast on objects
/// This is needed for the editor to extract the position of clicks on maps
///
/// You need to add this component to your camera to enable editing maps
#[derive(Component, Default)]
pub struct RayCamera {
    pub ray: Option<Ray>,
}
fn update_camera_ray(
    mut ray_cam: Query<(&mut RayCamera, &Camera, &GlobalTransform)>,
    mut cursor: EventReader<CursorMoved>,
) {
    let Some(cursor_position) = cursor.iter().last() else {
        return;
    };
    let cursor_position = cursor_position.position;
    let Ok((mut ray, cam, transform)) = ray_cam.get_single_mut() else {
        return;
    };
    let maybe_ray = cam.viewport_to_world(transform, cursor_position);
    if let Some(r) = maybe_ray {
        ray.ray = Some(r);
    } else {
        warn!("couldn't extract ray");
    }
}
