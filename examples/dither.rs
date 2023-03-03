use bevy::prelude::*;
use warbler_grass::dithering;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn)
        .add_system(on_load)
        .run();
}
#[derive(Resource)]
struct ImageHolder {
    pub image: Handle<Image>,
    pub dithered: bool,
}
fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let handle = asset_server.load("grass_height_map.png");
    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        transform: Transform::from_xyz(-300., 0., 0.),

        ..default()
    });
    commands.insert_resource(ImageHolder {
        image: handle,
        dithered: false,
    });
}

fn on_load(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut holder: ResMut<ImageHolder>,
) {
    if holder.dithered {
        return;
    }
    if let Some(image) = images.get(&holder.image) {
        info!("try to dither image");
        holder.dithered = true;
        let dithered_image = dithering::dither_image(image);
        if let Some(dithered_image) = dithered_image {
            let handle = images.add(dithered_image);
            commands.spawn(SpriteBundle {
                texture: handle.clone(),
                transform: Transform::from_xyz(200., 0., 0.),
                ..default()
            });
        } else {
            info!("returned none");
        }
    }
}
