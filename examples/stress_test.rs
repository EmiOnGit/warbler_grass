//! A stresstest to measure the performance of rendering a single huge chunk
//! Currently, around 10 million grass blades are loaded
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use warbler_grass::diagnostic::WarblerDiagnosticsPlugin;
use warbler_grass::prelude::*;
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // always needed to setup the grass render pipeline
        .add_plugin(WarblersPlugin)
        // helper plugin to spawn a camera which is moveable
        .add_plugin(helper::SimpleCamera)
        // creates our grass
        .add_startup_system(setup_grass)
        // Let's also log the amount of blades rendered
        // Since we spawn all grass in one huge chunk all blades get rendered
        // as long as one is on the screen (normally you'd devide the area into chunks)
        .add_plugin(WarblerDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load the image used for the height map
    let height_map_image = asset_server.load("grass_height_map.png");
    let height_map = HeightMap {
        height_map: height_map_image,
    };

    // load the image used for the density map
    let density_map_image = asset_server.load("grass_density_map.png");
    let density_map = DensityMap {
        density_map: density_map_image,
        // The density defines how many blades in a dense area spawns.
        density: 4.,
    };
    // spawn the entity rendering out large grass chunk
    commands.spawn(WarblersBundle {
        density_map,
        height_map,
        height: WarblerHeight::Uniform(5.),
        // Let's make a large chunk
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(1000., 100., 1000.)),
        ..default()
    });
}
