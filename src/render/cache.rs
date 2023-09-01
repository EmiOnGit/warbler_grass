use bevy::{prelude::*, render::render_resource::BindGroup};

#[derive(Resource, Default)]
pub(crate) struct UniformBuffer(Option<BindGroup>);
impl UniformBuffer {
    pub fn set(&mut self, val: BindGroup) {
        self.0 = Some(val);
    }
    pub fn ref_unwrap(&self) -> &BindGroup {
        self.0.as_ref().unwrap()
    }
}
