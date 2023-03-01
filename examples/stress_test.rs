use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use warbler_grass::{grass_spawner::GrassSpawner, prelude::*};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
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
