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

fn change_colors(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<RegionConfiguration>,
    time: Res<Time>,
) {
    let r = ((time.raw_elapsed_seconds() / 2.).sin() / 2.) + 0.5;
    let g = 1. - r;
    config.main_color.set_r(r);
    config.main_color.set_g(g);
    // if the right arrow key is pressed the color gets more blue
    if input.pressed(KeyCode::Right) {
        let b = config.main_color.b();

        config.main_color.set_b((b + 0.005).min(1.));
    }
    // if the left arrow key is pressed the color gets less blue
    if input.pressed(KeyCode::Left) {
        let b = config.main_color.b();
        config.main_color.set_b((b - 0.005).max(0.));
    }
}
