use bevy::prelude::*;
use warbler_grass::{
    grass_spawner::GrassSpawner, height_map::HeightMap, warblers_plugin::WarblersPlugin,
    WarblersBundle, editor::ray_cast,
};
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(WarblersPlugin)

        .add_plugin(helper::SimpleCamera)
        .add_plugin(ray_cast::RayCastPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map = asset_server.load("grass_height_map.png");

    let positions_xz: Vec<Vec2> = (0..10_000)
        .into_iter()
        .map(|i| (i / 100, i % 100))
        .map(|(x, z)| Vec2::new(x as f32, z as f32))
        .collect();
    let height_map = HeightMap {
        height_map,
        height: 5.,
    };
    let grass_spawner = GrassSpawner::new()
        .with_positions_xz(positions_xz)
        .with_height_map(height_map);
    commands.spawn(WarblersBundle {
        grass_spawner,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
        ..default()
    });
}
