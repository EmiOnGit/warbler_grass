use bevy::prelude::*;
use warblersneeds::{
    generator::{plane::Plane, GrassGenerator, StandardGeneratorConfig},
    warblers_plugin::WarblersPlugin,
    RegionConfiguration, WarblersBundle,
};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        // define the default color
        .insert_resource(RegionConfiguration {
            color: Color::rgb(0.5, 0.1, 0.0),
            ..default()
        })
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .add_system(change_colors)
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
    let plane1 = Plane {
        dimensions: Transform::from_xyz(30., 0., 30.),
    };

    let grass = plane1.generate_grass(config.clone());
    commands.spawn((WarblersBundle { grass, ..default() },));
}

fn change_colors(input: Res<Input<KeyCode>>, mut config: ResMut<RegionConfiguration>) {
    // if the right arrow key is pressed the color gets more green
    if input.pressed(KeyCode::Right) {
        let r = config.color.r();
        let g = config.color.g();
        config.color.set_r(r * 0.99);
        config.color.set_g(g * 1.01);
    }
    // if the left arrow key is pressed the color gets more red
    if input.pressed(KeyCode::Left) {
        let r = config.color.r();
        let g = config.color.g();
        config.color.set_r(r * 1.01);
        config.color.set_g(g * 0.99);
    }
}
