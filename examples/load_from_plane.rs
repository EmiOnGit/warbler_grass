use bevy::{
    prelude::*,
};
use warblersneeds::{
    generator::{
        plane::Plane,
        StandardGeneratorConfig, GrassGenerator,
    },
    warblers_plugin::WarblersPlugin,
    WarblersBundle,
};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
}
// In this example 2 planes are used for generating grass blades
fn setup_grass(mut commands: Commands) {
    let config = StandardGeneratorConfig {
        density: 10.,
        height: 3.,
        height_deviation: 0.5,
        seed: Some(0x121),
    };
    // translation indicates the outer point 
    let plane1 = Plane { dimensions: Transform::from_xyz(30., 0., 10.) };
    let plane2 = Plane { dimensions: Transform::from_xyz(10., 2., -10.) };

    let mut grass = plane1.generate_grass(config.clone());
    grass.0.extend(plane2.generate_grass(config).0);
    commands.spawn((
        WarblersBundle {
            grass,
            ..default()
        },
    ));
}
