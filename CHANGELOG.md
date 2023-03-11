# Changelog

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
* Grass chunks are updated now also updated if the height of the blades changed
* The frustum culling of chunks now works properly with big chunks
* A grass chunk is now set invisible if the parent is also invisible
* We don't crash the program if the density map is removed
* If hot reloading is activated, all maps are monitored
* Wind doesn't get faster over time
