# warbler_grass
Warbler is a common name for several birds. Warblers are not related biologically but by characteristics.
They are commonly small and vocal birds found in nature all over the globe. However, they couldn't sing their songs without having nature to comfort them.

This crate is meant to make the warblers happy by implementing a simple way to spawn grass in your [bevy](https://github.com/bevyengine/bevy) game.

## Warning
Don't use this for something serious. It's not ready to be used besides side projects or for learning purposes.

## Contributing
If you read this part, you might consider helping this project grow.
I consider this project very beginner friendly. 
It is relatively easy to grasp the workings since the use case is clear; to draw grass efficiently.
Don't fear if you are a beginner in bevy or even rust!

Currently, the code can be optimized in many places and many features I'd like to have been partially or completely missing.
You can always just create an issue and ask if something you want to do is needed.


![alt text](images/preview.png)
The preview image comes from [my demo project](https://github.com/EmiOnGit/birdylook) where I use this crate for grass rending

Another cool project is the [foxtrot](https://github.com/janhohenheim/foxtrot) template. Check it out!
## Example
```rust
use bevy::{prelude::*, render::primitives::Aabb};
use warbler_grass::{
    bundle::{WarblersBundle, WarblerHeight}, density_map::DensityMap, height_map::HeightMap,
    warblers_plugin::WarblersPlugin,
};
mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // This plugin is needed to initialize everything for the grass render pipeline
        .add_plugin(WarblersPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loading the height map from an image
    let height_map = asset_server.load("grass_height_map.png");
    // Constructing the height map struct
    let height_map = HeightMap { height_map };

    // Loading the density map from an image
    let density_map = asset_server.load("grass_density_map.png");
    // Constructing the density map
    let density_map = DensityMap {
        density_map,
        // The density corresponds to how dense a dense area is supposed to be.
        // Be careful with this parameter since the blade count grows fast. 
        density: 2.,
    };
    commands.spawn(WarblersBundle {
        height_map,
        density_map,
        // The height of the blades
        height: WarblerHeight::Uniform(2.),
        aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(100., 10., 100.)),
        ..default()
    });
}

```
