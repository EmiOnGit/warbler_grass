use bevy::prelude::*;
use warblersneeds::prelude::*;
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
fn setup_grass(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let config = StandardGeneratorConfig {
        density: 10.,
        height: 3.,
        height_deviation: 0.5,
        seed: Some(0x121),
    };
    // translation indicates the outer point
    let plane = Plane {
        dimensions: Transform::from_xyz(30., 0., 10.),
    };
    let grass = plane.generate_grass(config.clone());

    // The interesting part in this example
    // (The capsules would looks cool in the water!) :)
    let grass_mesh: Handle<Mesh> = meshes.add(shape::Capsule {
        radius: 0.1,
        ..default()
    }.into());
    // simple add the grass mesh in the warblersbundle, instead of using the default
    commands.spawn((WarblersBundle { grass, grass_mesh, ..default() },));
}
