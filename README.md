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
The preview image comes from [my demo project](https://github.com/EmiOnGit/birdylook) where I use this crate for grass rending

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
   let blades = (0..10000)
        .into_iter()
        .map(|i| GrassBlade {
            // making a grid
            position: Vec3::new(i / 100, 0., i % 100),
            height: i.ln(),
        })
        .collect();

    commands.spawn((WarblersBundle {
        grass: Grass::new(blades),
        ..default()
    },));
}

```
