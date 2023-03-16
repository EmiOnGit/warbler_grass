//! Demonstrates how to spawn multiple chunks
//!
//! Currently, the example spawns around 3 million grass blades in total
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use warbler_grass::{diagnostic::WarblerDiagnosticsPlugin, prelude::*};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        // Just a helper plugin for spawning a camera
        // As in all examples, you can use the wasd keys for movement and qe for rotation
        .add_plugin(helper::SimpleCamera)
        // Let's also log the amount of blades rendered
        .add_plugin(WarblerDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup_grass_chunks)
        .run();
}
fn setup_grass_chunks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let density_map_handle = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map: density_map_handle.clone(),
        density: 2.,
    };
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    // each chunk is 50x50
    let (chunk_width, chunk_height) = (50., 50.);
    // spawns a 20x20 grid of chunks
    for chunk in 0..400 {
        let offset = Vec3::new(
            (chunk / chunk_width as i32) as f32 * chunk_width * 1.05,
            0.,
            (chunk % chunk_width as i32) as f32 * chunk_height * 1.05,
        );
        commands.spawn(WarblersBundle {
            // we could use seperate density maps for each one
            density_map: density_map.clone(),
            // or seperate height maps if we wanted to
            height_map: height_map.clone(),
            height: WarblerHeight::Texture(density_map_handle.clone()),
            // the aabb defined the dimensions of the box the chunk lives in
            aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(chunk_width, 2., chunk_height)),

            spatial: SpatialBundle {
                transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        });
    }
}
