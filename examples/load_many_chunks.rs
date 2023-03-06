use bevy::prelude::*;
use warbler_grass::prelude::*;
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass_chunks)
        .run();
}
fn setup_grass_chunks(mut commands: Commands) {
    // in total we are loading 1_000_000 = 10_000 * 100 grass blades into the world
    let spawner = helper::get_grass_grid();

    for chunk in 0..100 {
        let offset = Vec3::new((chunk / 10) as f32 * 55., 0., (chunk % 10) as f32 * 55.);
        commands.spawn(WarblersBundle {
            grass_spawner: spawner.clone(),
            spatial: SpatialBundle {
                transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        });
    }
}
