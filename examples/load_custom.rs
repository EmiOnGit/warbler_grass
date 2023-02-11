use bevy::prelude::*;
use warblersneeds::{prelude::*, GrassBlade, Grass};
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
    let blades = (0..1000).into_iter()
        .map(|i| {
            let i = i as f32;
            (i.sin() * 20./ i.ln(), i.cos() * 20./ i.ln())
        }).map(|(x,z)| {
            GrassBlade { position: Vec3::new(x,0.,z), height: (x * x + z * z).ln() }
        }).collect();
    
    commands.spawn((
        WarblersBundle {
            grass: Grass(blades),
            ..default()
        },
    ));
}
