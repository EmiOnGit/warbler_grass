use bevy::prelude::*;
use warblersneeds::{
    file_loader::GrassFields,
    generator::GrassGenerator,
    prelude::{standard_generator::GrassFieldGenerator, StandardGeneratorConfig},
    warblers_plugin::WarblersPlugin,
    WarblersBundle,
};

mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup)
        .add_system(load_from_file)
        .run();
}
#[derive(Resource)]
struct GrassDataRes {
    data: Handle<GrassFields>,
    loaded: bool,
}
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let grass_data: Handle<GrassFields> = server.load("grass_placement.ron");
    let grass_res = GrassDataRes {
        data: grass_data,
        loaded: false,
    };
    commands.insert_resource(grass_res);
}

fn load_from_file(
    mut commands: Commands,
    mut grass_res: ResMut<GrassDataRes>,
    asset_loader: ResMut<Assets<GrassFields>>,
) {
    if !grass_res.loaded {
        if let Some(grass_data) = asset_loader.get(&grass_res.data) {
            let generator = GrassFieldGenerator { data: grass_data };

            let config = StandardGeneratorConfig {
                density: 10.,
                height: 2.,
                height_deviation: 0.5,
                seed: Some(0x121),
            };
            let grass = generator.generate_grass(config);

            commands.spawn((WarblersBundle {
                grass,
                transform: Transform::from_xyz(-150., 1., -10.),
                ..default()
            },));
            grass_res.loaded = true;
        }
    }
}
