use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use warbler_grass::{grass_spawner::GrassSpawner, prelude::*};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WarblersPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
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
fn setup_grass(mut commands: Commands) {
    let positions = (0..5_000_000)
        .into_iter()
        .map(|i| {
            let i = i as f32;
            (i % 1000., i / 1000.)
        })
        .map(|(x, z)| Vec3::new(x / 10., 2., z / 10.))
        .collect();

    commands.spawn((WarblersBundle {
        grass_spawner: GrassSpawner::new().with_positions(positions),
        ..default()
    },));
}
