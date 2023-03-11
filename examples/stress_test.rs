use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::primitives::Aabb,
    window::PresentMode,
};
use warbler_grass::prelude::*;
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::FpsPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        // Let's also log the amount of blades rendered
        // Since we spawn all grass in one huge chunk all blades get rendered
        // as long as one is on the screen (normally you'd devide the area into chunks) 
        .add_plugin(WarblerDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin)
        .run();
}

fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {

    let height_map_image = asset_server.load("grass_height_map.png");
    let height_map = HeightMap { height_map: height_map_image };

    let density_map_image = asset_server.load("grass_density_map.png");
    let density_map = DensityMap {
        density_map: density_map_image,
        density: 4.,
    };
    commands.spawn(WarblersBundle {
        density_map,
        height_map,
        height: WarblerHeight::Uniform(5.),
        // Let's make a large chunk
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(1000., 100., 1000.)),
        ..default()
    });
}
