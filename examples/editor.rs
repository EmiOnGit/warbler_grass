//! #NOTE
//! The editor is still worked on and can't be used currently
use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::editor::ray_cast::RayCamera;
use warbler_grass::{
    bundle::WarblersBundle, density_map::DensityMap, editor, height_map::HeightMap,
    warblers_plugin::WarblersPlugin,
};

mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_startup_system(setup_camera)
        .add_system(helper::camera_movement)
        // enable the editor by adding the plugin
        .add_plugin(editor::EditorPlugin)
        .add_system(refresh)
        .add_startup_system(setup_grass)
        .run();
}
#[derive(Component)]
struct Marker;
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>,     mut meshes: ResMut<Assets<Mesh>>,    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    let height_map = asset_server.load("grass_height_map.png");

    let height_map = HeightMap { height_map };
    let density_map_texture = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map: density_map_texture.clone(),
        density: 2.,
    };
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        10.,
        10.,
    ))));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(density_map.density_map.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 1.5),
        ..default()
    }).insert(Marker);
    commands.spawn(WarblersBundle {
        density_map,
        height_map,
        height: warbler_grass::prelude::WarblerHeight::Texture(density_map_texture),
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
                .looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
            ..default()
        },
        // The ray camera component is needed to find the positions of the press.
        RayCamera::default(),
    ));
}
fn refresh(
    mut commands: Commands,
    marked: Query<(Entity, &Handle<StandardMaterial>), With<Marker>>,
    density: Query<&DensityMap, Without<Marker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let (e,mat) = marked.single();
    let map = density.single();
    if let Some(mat) = materials.get_mut(&mat) {
        mat.base_color_texture = Some(map.density_map.clone());
    }
}