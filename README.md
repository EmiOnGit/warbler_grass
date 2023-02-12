# Warblersneeds
Warbler is a common name for several birds. Warblers are not related biologically but by characteristics.
They are commonly small and vocal birds found in nature all over the globe. However, they couldn't sing their songs without having nature to comfort them.

This crate is meant to make the warblers happy by implementing a simple way to spawn grass in your [bevy](https://github.com/bevyengine/bevy) game.
The crate is not mature and shouldn't be used seriously yet.
However, every type of contribution is currently highly appreciated, so feel free to open an issue with your suggestion!

The code base is very young and will be redesigned soon.
I'd love it if you would tell me your thoughts on how the redesign might look like. Make sure to comment on the [issue I created for that](https://github.com/EmiOnGit/warblersneeds/issues/1)

The current implementation is fairly performant. The stress test with 1_000_000 grass blades animated with wind currently runs at around 250 fps on my hardware.
This is because most of the work is done on the GPU and the grass is instanced properly.

If you want to look at examples, you can take a look at the [examples](./examples/) folder of this project

![alt text](images/preview.png)

## Example
```rust
use bevy::prelude::*;
use warblersneeds::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands) {
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
   
    // create the grass from the plane generator
    let grass = plane.generate_grass(config.clone());
    
    // spawn the grass into the world
    commands.spawn((WarblersBundle { grass, ..default() },));
}

```