use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode, render::primitives::Aabb,
};
use warbler_grass::{prelude::*, height_map::HeightMap, density_map::DensityMap, bundle::WarblersBundle};
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
        .add_startup_system(setup_fps)
        .add_system(diagnostic_system)
        .run();
}
pub fn setup_fps(mut diagnostics: ResMut<Diagnostics>) {
    diagnostics.add(Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS, "fps", 200));
}
pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, time: Res<Time>) {
    let delta_seconds = time.raw_delta_seconds_f64();
    if delta_seconds == 0.0 {
        return;
    }
    diagnostics.add_measurement(FrameTimeDiagnosticsPlugin::FPS, || 1.0 / delta_seconds);
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let density_map = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map,
        density: 2.,
    };
    commands.spawn(WarblersBundle {
        density_map,
        height_map,
        height: bundle::WarblerHeight::Uniform(5.),
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(2000., 100., 2000.)),
        ..default()
    });
}
