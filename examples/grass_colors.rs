use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::prelude::*;
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        // Just a helper plugin for spawning a camera
        // As in all examples, you can use the wasd keys for movement and qe for rotation
        .add_plugin(helper::SimpleCamera)
        // We can define the color at startup if we want to
        .insert_resource(GrassConfiguration {
            // The color of the upper part of the grass blades
            main_color: Color::WHITE,
            // The color of the lower part
            bottom_color: Color::ALICE_BLUE,
            ..default()
        })
        .add_startup_system(setup_grass)
        .add_system(change_colors)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let density_map = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map,
        density: 2.,
    };
    commands.spawn((WarblersBundle {
        density_map,
        height_map,
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 10., 100.)),
        ..default()
    },));
}
// we can also change the color over at other times
// this can be useful if your game has seasons
fn change_colors(mut config: ResMut<GrassConfiguration>, time: Res<Time>) {
    // Most likely you'd want to choose other colors
    let r = ((time.raw_elapsed_seconds() / 2.).sin() / 2.) + 0.5;
    let g = 1. - r;
    config.main_color.set_r(r);
    config.main_color.set_g(g);
    config.main_color.set_b((g * r).sin());
}
