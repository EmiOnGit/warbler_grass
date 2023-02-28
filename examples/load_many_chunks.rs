use bevy::prelude::*;
use warblersneeds::prelude::*;
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
    // in total we are loading 10_000 = 100 * 100 grass blades into the world
    let spawner = helper::get_grass_grid();

    for chunk in 0..100 {
        let offset = Vec3::new((chunk / 10) as f32 * 11., 0., (chunk % 10) as f32 * 11.);
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
