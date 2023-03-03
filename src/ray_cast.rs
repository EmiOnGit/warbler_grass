pub use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_raycast::{RaycastMesh, Intersection, DefaultPluginState, RaycastSource, RaycastMethod, RaycastSystem, Primitive3d, Ray3d, PrimitiveIntersection};

use crate::grass_spawner::GrassSpawner;
pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
        .add_system(add_ray_caster)
        // .add_system(intersection)       
        .add_system(check_path)       
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
        );
    }
}
pub struct MyRaycastSet;
fn startup(mut commands: Commands) {
    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
}
fn add_ray_caster(grass_iter: Query<(Entity, &Aabb), (Added<Aabb>,With<GrassSpawner>)>, mut commands: Commands) {
    // commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
    for (entity,aabb) in grass_iter.iter() {
        let point = aabb.center.as_dvec3().as_vec3();
        commands.entity(entity)
            .insert(RaycastMesh::<MyRaycastSet>::default());
    }
}

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
fn check_path(
    mut from: Query<
        (&mut Transform, &Aabb), (With<GrassSpawner>, Without<RaycastSource<MyRaycastSet>>),
        >,
    camera_source: Query<(&Transform, &RaycastSource<MyRaycastSet>)>,
) {
    for  (origin_transform, aabb) in from.iter_mut() {
        let from = origin_transform.translation;
        let (to, _raycast_source) = camera_source.single();
        let to = to.translation;
        let _ray_direction = (to - from).normalize();
        let ray = Ray3d::new(to, Vec3::NEG_Y);
        let aabb_center = aabb.center.as_dvec3().as_vec3() + from;
        let grass_plane = Primitive3d::Plane { point: aabb_center , normal: Vec3::Y };
        let res = ray.intersects_primitive(grass_plane);
        match res {
            Some(intersection) => {
                println!("center: {}", aabb_center);
                println!("distance: {}", intersection.position() - aabb_center);
                println!("positions: {}", intersection.position());
            }
            None => {
                println!("none");
            }
        }
    }
    
    // println!("ray {res:?}");
}