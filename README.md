# warbler_grass
Warbler is a common name for several birds. Warblers are not related biologically but by characteristics.
They are commonly small and vocal birds found in nature all over the globe. However, they couldn't sing their songs without having nature to comfort them.

This crate is meant to make the warblers happy by implementing a simple way to spawn grass in your [bevy](https://github.com/bevyengine/bevy) game.
The crate is not mature and shouldn't be used seriously yet.
However, every type of contribution is currently highly appreciated, so feel free to open an issue with your suggestion!

## Warning
The code is very young and surely still changes now and then.

When the base is suifficently figured out, however, I will swap to making proper notes on changes I do, as well as using versioning.

## Contributing
If you read this part, you might consider helping this project grow.
I consider this project very beginner friendly. 
It is relatively easy to grasp the workings since the use case is clear; to draw grass efficiently.
Don't fear if you are a beginner in bevy or even rust!

Currently, the code can be optimized in many places and many features I'd like to have been partially or completely missing.
You can always just create an issue and ask if something you want to do is needed.
I'll try to make issues in advance about the larger topics I'll want to be implemented.
Look at the [milestones](https://github.com/emiongit/warbler_grass/milestones) for inspiration.

## Performance
The current implementation is already "fairly" performant. 
The stress test with 5_000_000 grass blades animated with wind currently runs at around 350 fps on my hardware.
This has been achieved by extensively using the GPU, proper instancing, as well as caching.
However, there are still many improvements to be made.

If you want to look at examples, you can take a look at the [examples](./examples/) folder of this project

![alt text](images/preview.png)
The preview image comes from [my demo project](https://github.com/EmiOnGit/birdylook) where I use this crate for grass rending

## Example
```rust
use bevy::prelude::*;
use warblers_grass::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WarblersPlugin)
        .add_startup_system(setup_grass)
        .run();
}
fn setup_grass(mut commands: Commands) {
   let blades = (0..10_000)
        .into_iter()
        .map(|i| GrassBlade {
            // making a grid
            position: Vec3::new(i / 100, 0., i % 100),
            height: i.ln(),
        })
        .collect();

    commands.spawn((WarblersBundle {
        grass_spawner: GrassSpawner::new().from_grass_blades(blades),
        ..default()
    },));
}

```
