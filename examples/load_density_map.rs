use bevy::{prelude::*, window::PresentMode};
use warbler_grass::{
    density_map::DensityMap, editor, grass_spawner::GrassSpawner, height_map::HeightMap,
    warblers_plugin::WarblersPlugin, WarblersBundle,
};
mod helper;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::FpsPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_plugin(editor::EditorPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map = asset_server.load("grass_height_map.png");
    let density_map = asset_server.load("grass_density_map.png");

    let mut height_map = HeightMap {
        height_map,
        height: 1.,
    };
    let density_map = DensityMap {
        density_map,
        span_xz: Vec2::ONE * 64.,
        density: 4.,
        noise: true,
    };
    let grass_spawner = GrassSpawner::new()
        .with_density_map(density_map)
        .with_height_uniform(4.5)
        .with_height_map(height_map.clone());

    commands.spawn(WarblersBundle {
        grass_spawner,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 3., 0.),
            ..default()
        },
        ..default()
    });
    height_map.height = 30.;
    let positions_xz: Vec<Vec2> = (0..1_000_000)
        .into_iter()
        .map(|i| (i / 1000, i % 1000))
        .map(|(x, z)| Vec2::new(x as f32, z as f32) / 1.5)
        .collect();
    
    let grass_spawner = GrassSpawner::new()
        .with_positions_xz(positions_xz)
        .with_height_uniform(4.5)

        .with_height_map(height_map);
    commands.spawn(WarblersBundle {
        grass_spawner,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 3., -650.),
            ..default()
        },
        ..default()
    });
}
