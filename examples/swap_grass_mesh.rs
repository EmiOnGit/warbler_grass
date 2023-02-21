use bevy::prelude::*;
use warblersneeds::{prelude::*, warblers_plugin::GRASS_MESH_HANDLE};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .add_system(swap_grass_mesh)
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
    // Normally the grass mesh should start at y>=0.
    // Fixing this would make the grass look even nicer!
    let grass_mesh: Handle<Mesh> = meshes.add(
        shape::Capsule {
            radius: 0.1,
            depth: 0.5,
            ..default()
        }
        .into(),
    );

    let store = GrassMeshStore {
        custom_handle: grass_mesh.clone(),
        default_handle: GRASS_MESH_HANDLE.typed(),
    };
    commands.insert_resource(store);
    // simple add the grass mesh in the warblersbundle, instead of using the default
    commands.spawn((WarblersBundle {
        grass,
        grass_mesh,
        ..default()
    },));
}
#[derive(Resource)]
struct GrassMeshStore {
    pub custom_handle: Handle<Mesh>,
    pub default_handle: Handle<Mesh>,
}
// press tab to swap meshes
fn swap_grass_mesh(
    mut commands: Commands,
    mut queue: Query<(Entity, &mut Handle<Mesh>), With<Grass>>,
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
