//! Shows how to swap out the default grass mesh.
//! You can use the TAB key to swap between the new mesh and the default grass mesh

use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::{prelude::*, warblers_plugin::GRASS_MESH_HANDLE};
mod helper;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // This plugin is needed to initialize everything for the grass render pipeline
            WarblersPlugin,
            // Just a helper plugin for spawning a camera
            // As in all examples, you can use the wasd keys for movement and qe for rotation
            helper::SimpleCamera,
        ))
        .add_systems(Startup, setup_grass)
        .add_systems(Update, swap_grass_mesh)
        .run();
}

fn setup_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // The interesting part in this example
    // We could use any mesh we want. We should choose a low poly mesh however since we draw a lot of them
    let grass_mesh: Handle<Mesh> = meshes.add(
        shape::Capsule {
            radius: 0.3,
            depth: 2.5,
            ..default()
        }
        .into(),
    );
    // we use a resource to keep track of the handles
    // so we can swap them later
    let store = GrassMeshStore {
        custom_handle: grass_mesh.clone(),
        default_handle: GRASS_MESH_HANDLE.typed(),
    };
    commands.insert_resource(store);

    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let density_map = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map,
        density: 1.,
    };
    // simple add the grass mesh in the bundle, instead of using the default
    commands.spawn(WarblersBundle {
        grass_mesh,
        density_map,
        height_map,
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 10., 100.)),
        ..default()
    });
}

/// Used to keep track of the standard mesh and the custom mesh for the grass
#[derive(Resource)]
struct GrassMeshStore {
    pub custom_handle: Handle<Mesh>,
    pub default_handle: Handle<Mesh>,
}

// Swapps the mesh type if TAB is pressed
fn swap_grass_mesh(
    mut commands: Commands,
    mut queue: Query<(Entity, &mut Handle<Mesh>)>,
    input: Res<Input<KeyCode>>,
    store: Res<GrassMeshStore>,
) {
    if input.just_pressed(KeyCode::Tab) {
        for (e, mesh_handle) in queue.iter_mut() {
            if store.default_handle.id() == mesh_handle.id() {
                commands.entity(e).insert(store.custom_handle.clone());
            } else if store.custom_handle.id() == mesh_handle.id() {
                commands.entity(e).insert(store.default_handle.clone());
            }
        }
    }
}
