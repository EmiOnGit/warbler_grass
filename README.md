# Warblersneeds
Warbler is a common name for several birds. Warblers are not related biologically but by characteristics.
They are commonly small and vocal birds found in nature all over the globe. However, they couldn't sing their songs without having nature to comfort them.

This crate is meant to make the warblers happy by implementing a simple way to spawn grass in your [bevy](https://github.com/bevyengine/bevy) game.
The crate is not mature and shouldn't be used seriously yet.
However, every type of contribution is currently highly appreciated, so feel free to open an issue with your suggestion!

The current implementation is fairly performant. 
The stresstest with 1_000_000 grass blades animated with wind currently runs at around 250 fps on my hardware.
This is because most of the work is done on the gpu and the grass is instanced properly.

If you want to look at examples, you can take a look at the [examples](./examples/) folder of this project

![alt text](images/preview.png)
