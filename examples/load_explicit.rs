//! Demonstrates how to spawn grass blades using explicit positions.
//! In this example we spawn the blades an a golden ration spiral
use bevy::prelude::*;
use warbler_grass::prelude::*;
mod helper;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // always needed to create the grass render pipeline
        .add_plugin(WarblersPlugin)
        // spawns a simple camera which can be moved
        .add_plugin(helper::SimpleCamera)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands) {
    // The positions of each blade (relative from entity transform)
    let count = 2000;
    let radius = 6.;
    let speed = 3.;
    let positions = (0..count)
        .into_iter()
        // the part that makes our spiral
        .map(|i| {
            let f = i as f32 / count as f32 * speed;
            let dist = f * radius;
            let x = 0.5 + (f * std::f32::consts::PI * 2.).cos() * dist;
            let y = 0.5 + (f * std::f32::consts::PI * 2.).sin() * dist;
            (x, y)
        })
        // define our positions
        .map(|(x, y)| Vec3::new(x, 0., y))
        // collect the positions to a vector
        .collect();

    // If we want to use explicit positions, we need to use
    // the WarblersExplicitBundle instead of the normal WarblersBundle
    commands.spawn(WarblersExplicitBundle {
        grass: Grass {
            // the positions of the grass blades
            positions,
            // the height of the blades
            height: 2.,
        },
        ..default()
    });
}
