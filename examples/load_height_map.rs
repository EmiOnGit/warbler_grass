use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastMesh, RaycastSource, Intersection, DefaultPluginState};
use warbler_grass::{
    grass_spawner::GrassSpawner, height_map::HeightMap, warblers_plugin::WarblersPlugin,
    WarblersBundle, ray_cast::{self, MyRaycastSet},
};
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_plugin(WarblersPlugin)

        .add_plugin(helper::SimpleCamera)
        .add_plugin(ray_cast::RayCastPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>,   mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,) {
    let height_map = asset_server.load("grass_height_map.png");

    let positions_xz: Vec<Vec2> = (0..100)
        .into_iter()
        .map(|i| (i / 10, i % 10))
        .map(|(x, z)| Vec2::new(x as f32, z as f32))
        .collect();
    let height_map = HeightMap {
        height_map,
        height: 10.,
    };
    let grass_spawner = GrassSpawner::new()
        .with_positions_xz(positions_xz)
        .with_height_map(height_map);
    commands.spawn(WarblersBundle {
        grass_spawner,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 1., 20.),
            ..default()
        },
        ..default()
    });
    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());


    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
            ..Default::default()
        },
        RaycastMesh::<MyRaycastSet>::default(), // Make this mesh ray cast-able
    ));
}


// fn rotator(time: Res<Time>, mut query: Query<&mut Transform, With<RaycastSource<MyRaycastSet>>>) {
//     for mut transform in &mut query {
//         *transform = Transform::from_rotation(
//             Quat::from_rotation_x(time.elapsed_seconds().sin() as f32 * 0.2)
//                 * Quat::from_rotation_y((time.elapsed_seconds() * 1.5).sin() as f32 * 0.1),
//         );
//     }
// }