use bevy::{prelude::*, render::view::NoFrustumCulling};
use warblersneeds::{prelude::*, grass_spawner::GrassSpawner};
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands) {
    // we can define our blades how we want
    let (positions, heights) = (0..1000)
        .into_iter()
        .map(|i| {
            let i = i as f32;
            (i.sin() * 20. / i.ln(), i.cos() * 20. / i.ln())
        })
        .map(|(x, z)| 
           (Vec3::new(x, 0., z),
            (x * x + z * z).ln()))
        .unzip();
    let grass_spawner = GrassSpawner::new()
        .with_positions(positions)
        .with_heights(heights);
    commands.spawn((WarblersBundle {
        grass_spawner,
        ..default()
    },NoFrustumCulling));
}
