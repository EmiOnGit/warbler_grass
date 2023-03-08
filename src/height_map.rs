use bevy::prelude::*;

#[derive(Reflect, Clone, Component)]
pub struct HeightMap {
    pub height_map: Handle<Image>,
}
impl From<Handle<Image>> for HeightMap {
    fn from(value: Handle<Image>) -> Self {
        HeightMap { height_map: value }
    }
}
