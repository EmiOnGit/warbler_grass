//! The basic example which simply spawns a chunk of grass the recommended way
use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::prelude::*;
mod helper;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // This plugin is needed to initialize everything for the grass render pipeline
            WarblersPlugin,
            // Just a helper plugin for spawning a camera
            // As in all examples, you can use the wasd keys for movement and qe for rotation
            helper::SimpleCamera,
        ))
        .add_systems(Startup, setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loading the y-map texture
    let y_map_image = asset_server.load("grass_y_map.png");
    // Constructing the y-map struct
    let y_map = YMap { y_map: y_map_image };

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
        y_map,
        density_map,
        // The height of the blades
        height: WarblerHeight::Uniform(2.),
        // The aabb defines the area in which the chunk lives in
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 5., 100.)),
        ..default()
    });
}
