use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, window::PresentMode};
use warblersneeds::{prelude::*, GrassBlade, Grass};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: PresentMode::Immediate,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WarblersPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands) {
    let blades = (0..1_000_000).into_iter()
        .map(|i| {
            let i = i as f32;
            (i % 1000., i / 1000.)
        }).map(|(x,z)| {
            GrassBlade { position: Vec3::new(x / 10.,0.,z / 10.), height: 1. }
        }).collect();
    
    commands.spawn((
        WarblersBundle {
            grass: Grass(blades),
            ..default()
        },
    ));
}
