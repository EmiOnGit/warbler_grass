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
    // in total we are loading 250_000 = 2_500 * 100 grass blades into the world
    let blades: Vec<GrassBlade> = (0..2_500)
        .into_iter()
        .map(|i| {
            let i = i as f32;
            (i % 50., i / 50.)
        })
        .map(|(x, z)| GrassBlade {
            position: Vec3::new(x / 10., 2., z / 10.),
            height: ((x.sin() + z.sin()).cos() + 5.) / 10.,
        })
        .collect();

    for chunk in 0..100 {
        let offset = Vec3::new((chunk / 10) as f32 * 6., 0., (chunk % 10) as f32 * 6.);
        commands.spawn(WarblersBundle {
            grass: Grass::new(blades.clone()),
            spatial: SpatialBundle {
                transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        });
    }
}
