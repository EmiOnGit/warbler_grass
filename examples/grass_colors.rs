use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::{warblers_plugin::WarblersPlugin, GrassConfiguration, height_map::HeightMap, density_map::DensityMap, bundle::WarblersBundle};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
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

fn change_colors(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<GrassConfiguration>,
    time: Res<Time>,
) {
    let r = ((time.raw_elapsed_seconds() / 2.).sin() / 2.) + 0.5;
    let g = 1. - r;
    config.main_color.set_r(r);
    config.main_color.set_g(g);
    // if the right arrow key is pressed the color gets more blue
    if input.pressed(KeyCode::Right) {
        let b = config.main_color.b();

        config.main_color.set_b((b + 0.005).min(1.));
    }
    // if the left arrow key is pressed the color gets less blue
    if input.pressed(KeyCode::Left) {
        let b = config.main_color.b();
        config.main_color.set_b((b - 0.005).max(0.));
    }
}
