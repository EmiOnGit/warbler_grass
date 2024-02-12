# Changelog
## 0.5
* Support for bevy 0.12
* `WarblerBundle` now contains the `NoAutomaticBatching` component
Besides that, a lot of internal restructering was needed

## 0.4
* Support for bevy 0.11
* The color of the grass is now a `Component`, meaning it can be configured on a `Chunk` basis.
* Remove the `WarblersExplicitBundle`. Only textures are now supported.
* `HeightMap` is now named `YMap` to easier distinguish between `WarblerHeights`, which controls the height of the grass blades.
* The `maps` module is now named `map` (which contains the `YMap` and `DensityMap` components). 
## 0.3.2
This release mainly includes proper support for wasm builds,
as well as simplifications in the code and better documentation.
Also the performance should be slightly better for the editor.

No migration should be needed but to update the version

## 0.3.1
This version shouldn't need any work on the user side.

All changes were intern or bug fixes (see #54, #56, #52)

Also the dev features were intruduced so running the examples is easier

Besides that, I installed inkscape and tried to paint a logo.. which is now the officially on the `warbler_grass` github repo ! :D

## 0.3
### Change
* The `GrassSpawner` was removed in favor of the `WarblersBundle` and `WarblersExplicitBundle`
* The `wind_noise_texture` field of the `GrassConfiguration` resource was moved to the `GrassNoiseTexture` resource
* The `density_map` and `height_map` modules were combined to the `maps` module
* Dithering uses a new matrix which changes the positions of your grass blades slightly if you used a density map before

### Added
* A editor was implemented
This makes it possible to modify your maps directly in the game and save them after
The editor can only be used with the new `editor` feature (you can take a look at the editor example)
* A `WarblerDiagnosticsPlugin` has been added which logs the amount of grass bladed rendered every frame
This is used in the `many_chunks` exampe if you'd like to see how to use it
* The height of the blades can now be sampled using a texture with the `WarblersHeight::Texture` component
* A density parameter can now also be specified with the `DensityMap` to directly influence the dithering
* The default wind direction was changed from (1,0) to (1,1)

### Fix
Many bugs have been fixed; to many to count all of them
* Warbler_grass only uses the dependencies it needs!
* Grass chunks are updated now also updated if the height of the blades changed
* The frustum culling of chunks now works properly with big chunks
* A grass chunk is now set invisible if the parent is also invisible
* We don't crash the program if the density map is removed
* If hot reloading is activated, all maps are monitored
* Wind doesn't get faster over time
