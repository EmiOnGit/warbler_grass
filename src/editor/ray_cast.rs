pub use bevy::prelude::*;
use bevy::{render::primitives::Aabb, math::Vec3Swizzles};
use bevy_mod_raycast::{RaycastSource, RaycastMethod, RaycastSystem, Primitive3d, DefaultRaycastingPlugin};

use crate::grass_spawner::GrassSpawner;

use super::draw_event::{DrawEvent, ActiveBrush, Stencil, draw_map};
pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
            .add_system(check_collision_on_click)
            .insert_resource(ActiveBrush::new(Stencil::default()))  
            .add_event::<DrawEvent>()
            .add_system(draw_map)
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
            );
    }
}
pub struct MyRaycastSet;

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}
fn check_collision_on_click (
    mut grass_chunk: Query<
        (&mut Transform, &Aabb, &GrassSpawner), (With<GrassSpawner>, Without<RaycastSource<MyRaycastSet>>),
        >,
    camera_source: Query<(&Transform, &RaycastSource<MyRaycastSet>)>,
    mouse_presses: Res<Input<MouseButton>>,
    mut draw_events: EventWriter<DrawEvent>
) {
    if !mouse_presses.pressed(MouseButton::Left) {
        return;
    }
    let (_camera_transform, raycast_camera) = camera_source.single();
    let click_ray = raycast_camera.get_ray().unwrap();
    for  (chunk_transform, aabb, grass) in grass_chunk.iter_mut() {

        let aabb_center = aabb.center.as_dvec3().as_vec3() + chunk_transform.translation;

        let grass_plane = Primitive3d::Plane { point: aabb_center , normal: Vec3::Y };
        let res = click_ray.intersects_primitive(grass_plane).unwrap();
        let intersection_point = (res.position() - aabb_center).xz();
        let aabb_extends = aabb.half_extents.as_dvec3().as_vec3().xz().abs();
        if aabb_extends.x > intersection_point.x 
        && -aabb_extends.x < intersection_point.x 
        && aabb_extends.y > intersection_point.y 
        && -aabb_extends.y < intersection_point.y {
            let positions = (Vec2::new(intersection_point.x / aabb_extends.x, intersection_point.y / aabb_extends.y) + Vec2::ONE) / 2.;
            let image = grass.height_map.as_ref().unwrap().height_map.clone();
            draw_events.send(DrawEvent::Draw { positions, image});
        }
    }
}   
