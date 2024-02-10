//! A stresstest to measure the performance of rendering a single huge chunk
//! Currently, around 10 million grass blades are loaded
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use warbler_grass::diagnostic::WarblerDiagnosticsPlugin;
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
            // Let's also log the amount of blades rendered
            // Since we spawn all grass in one huge chunk all blades get rendered
            // as long as one is on the screen (normally you'd devide the area into chunks)
            WarblerDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        // creates our grass
        .add_systems(Startup, setup_grass)
        // more wind
        .insert_resource(GrassConfiguration {
            wind: Vec2::new(2., 2.),
        })
        .run();
}

fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load the image used for the y-map
    let y_map_image = asset_server.load("grass_y_map.png");
    let y_map = YMap { y_map: y_map_image };

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
        y_map,
        height: WarblerHeight::Uniform(5.),
        // Let's make a large chunk
        // With our density map we spawn around 10 million blades on this area
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(1000., 0., 1000.)),
        spatial: SpatialBundle {
            // translate the chunk so we are in a nice middle place
            transform: Transform::from_xyz(-480., -5., -480.),
            ..default()
        },
        ..default()
    });
}
