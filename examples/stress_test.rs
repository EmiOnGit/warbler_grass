use bevy::{
    prelude::*,
    window::PresentMode,
};
use warbler_grass::{grass_spawner::GrassSpawner, prelude::*};
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
        .run();
}

fn setup_grass(mut commands: Commands) {
    let positions = (0..10_000_000)
        .into_iter()
        .map(|i| {
            let i = i as f32;
            (i % 2000., i / 1000.)
        })
        .map(|(x, z)| Vec3::new(x / 10., 2., z / 10.))
        .collect();

    commands.spawn((WarblersBundle {
        grass_spawner: GrassSpawner::new().with_positions(positions),
        ..default()
    },));
}
