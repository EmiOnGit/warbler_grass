use bevy::prelude::*;


#[derive(Reflect, Clone)]
pub struct HeightMap {
    pub height_map: Handle<Image>,
    pub height: f32,
}
