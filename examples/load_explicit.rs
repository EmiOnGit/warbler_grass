use bevy::prelude::*;
use warbler_grass::prelude::*;
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
    // The positions of each blade (relative from entity transform)
    let positions = (0..1000)
        .into_iter()
        .map(|i| {
            let i = i as f32;
            // let's make them circly
            (i.sin() * 20. / i.ln(), i.cos() * 20. / i.ln())
        })
        .map(|(x, z)| Vec3::new(x, x * z / 10., z))
        .collect();

    commands.spawn(WarblersExplicitBundle {
        grass: Grass {
            positions,
            height: 2.,
        },
        ..default()
    });
}
