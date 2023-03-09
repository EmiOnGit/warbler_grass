use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::prelude::*;
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass_chunks)
        .run();
}
fn setup_grass_chunks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let density_map = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map,
        density: 2.,
    };
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let (chunk_width, chunk_height) = (40, 40);
    for chunk in 0..400 {
        let offset = Vec3::new(
            (chunk / chunk_width) as f32 * chunk_width as f32 * 1.05,
            0.,
            (chunk % chunk_width) as f32 * chunk_height as f32 * 1.05,
        );
        commands.spawn(WarblersBundle {
            density_map: density_map.clone(),
            height_map: height_map.clone(),
            aabb: Aabb::from_min_max(
                Vec3::ZERO,
                Vec3::new(chunk_width as f32, 2., chunk_height as f32),
            ),
            spatial: SpatialBundle {
                transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        });
    }
}
