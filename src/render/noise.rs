use bevy::prelude::*;
#[derive(Resource)]
pub(crate) struct NoiseTexture {
    pub texture: Handle<Image>,
}
impl NoiseTexture {
    pub fn new(texture: Handle<Image>) -> Self {
        NoiseTexture { texture }
    }
}
