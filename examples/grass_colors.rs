use bevy::{color::palettes, prelude::*, render::primitives::Aabb};
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
        .add_systems(Update, change_colors)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let y_map_image = asset_server.load("grass_y_map.png");

    let y_map = YMap { y_map: y_map_image };
    let density_map = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map,
        density: 2.,
    };
    commands.spawn((WarblersBundle {
        density_map,
        y_map,
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 10., 100.)),
        // you can define the color for each grass chunk
        grass_color: GrassColor {
            main_color: (palettes::css::DARK_GREEN).into(),
            bottom_color: (palettes::css::DARK_GREEN * 0.5).into(),
        },
        ..default()
    },));
}
// we can also change the color over at other times
// this can be useful if your game has seasons
fn change_colors(mut grass_colors: Query<&mut GrassColor>, time: Res<Time>) {
    // Most likely you'd want to choose other colors
    let r = ((time.elapsed_seconds() / 2.).sin() / 2.) + 0.5;
    let g = 1. - r;
    for mut color in &mut grass_colors {
        color.main_color = Color::srgb(r, g, (g * r).sin()).into();
        // the bottom color should normally be a bit darker than the main color.
        color.bottom_color = (color.main_color.to_linear() * 0.5).into();
    }
}
