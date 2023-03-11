//! The basic example which simply spawns a chunk of grass the recommended way
use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::prelude::*;
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // This plugin is needed to initialize everything for the grass render pipeline
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loading the height map from an image
    let height_map = asset_server.load("grass_height_map.png");
    // Constructing the height map struct
    let height_map = HeightMap { height_map };

    // Loading the density map from an image
    let density_map = asset_server.load("grass_density_map.png");
    // Constructing the density map
    let density_map = DensityMap {
        density_map,
        // The density corresponds to how dense a dense area is supposed to be
        // Be careful with this parameter since the blade count grows fast
        density: 2.,
    };
    // spawns the "chunk" entity
    commands.spawn(WarblersBundle {
        height_map,
        density_map,
        // The height of the blades
        height: WarblerHeight::Uniform(2.),
        // The aabb defines the area in which the chunk lives in
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 5., 100.)),
        ..default()
    });
}
