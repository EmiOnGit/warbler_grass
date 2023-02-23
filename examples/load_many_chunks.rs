use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use warblersneeds::prelude::*;
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
        .add_startup_system(setup_grass_chunks)
        .run();
}
fn setup_grass_chunks(mut commands: Commands) {
    let blades: Vec<GrassBlade> = (0..10_000)
            .into_iter()
            .map(|i| {
                let i = i as f32;
                (i % 100., i / 100.)
            })
            .map(|(x, z)| GrassBlade {
                position: Vec3::new(x / 10., 2., z / 10.),
                height: ((x.sin() + z.sin()).cos() + 5.) / 10.,
            })
            .collect();
    for chunk in 0..100 {
        let offset = Vec3::new((chunk / 10) as f32 * 12., 0., (chunk  % 10) as f32 * 12.);
        commands.spawn(WarblersBundle {
            grass: Grass::new(blades.clone()),
            spatial: SpatialBundle{ transform: Transform::from_translation(offset), ..default() },
            ..default()
        });
    }
    
}
