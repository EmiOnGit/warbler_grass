use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use warbler_grass::editor::ray_cast::{RayCamera, SelectedMap};
use warbler_grass::editor::{self, ActiveEditorChunk};
use warbler_grass::prelude::*;
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_startup_system(setup_camera)
        // As in all examples, you can use the wasd keys for movement and qe for rotation
        .add_system(helper::camera_movement)
        // enable the editor by adding the plugin
        .add_plugin(editor::EditorPlugin {
            // You can choose the scale factor for the ui or use the default 
            scale_factor: 1.1
        })
        .add_system(refresh_texture_view)
        .add_startup_system(setup_grass)
        .run();
}
#[derive(Component)]
struct Marker;
fn setup_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let density_map_texture = asset_server.load("grass_density_map.png");
    let heights_map_texture = asset_server.load("grass_heights_map.png");

    let density_map = DensityMap {
        density_map: density_map_texture.clone(),
        density: 2.,
    };
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10., 10.))));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(density_map.density_map.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    commands
        .spawn(PbrBundle {
            mesh: quad_handle.clone(),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5),
            ..default()
        })
        .insert(Marker);
    commands.spawn(WarblersBundle {
        density_map,
        height_map,
        height: warbler_grass::prelude::WarblerHeight::Texture(heights_map_texture.clone()),
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 5., 100.)),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
        ..default()
    });
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15., 55.0)
                .looking_at(Vec3::new(10., 10., 55.), Vec3::Y),
            ..default()
        },
        // The ray camera component is needed to find the positions of the press.
        RayCamera::default(),
    ));
}
fn refresh_texture_view(
    marked: Query<&Handle<StandardMaterial>, With<Marker>>,
    chunk: Query<(&DensityMap, &HeightMap, &WarblerHeight), Without<Marker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected_map: Res<SelectedMap>,
    active_chunk: Res<ActiveEditorChunk>,
) {
    let Some(active_entity) = active_chunk.0 else {
        return;
    };
    let material = marked.single();
    let Ok((density_map, height_map, heights)) = chunk.get(active_entity) else {
        return;
    };
    if let Some(mat) = materials.get_mut(&material) {
        match *selected_map {
            SelectedMap::HeightMap => mat.base_color_texture = Some(height_map.height_map.clone()),
            SelectedMap::DensityMap => {
                mat.base_color_texture = Some(density_map.density_map.clone())
            }
            SelectedMap::HeightsMap => {
                if let WarblerHeight::Texture(heights_map) = heights {
                    mat.base_color_texture = Some(heights_map.clone())
                } else {
                    mat.base_color_texture = Some(density_map.density_map.clone())
                }
            }
        }
    }
}
