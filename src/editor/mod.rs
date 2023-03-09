use bevy::prelude::Plugin;

use self::{
    brush::ActiveBrush,
    draw_event::{draw_map, DrawEvent},
    hot_reloading::notify_image_change,
    ray_cast::RayCastPlugin,
};
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::editor::brush::Brushes;
use bevy::prelude::KeyCode;
pub mod brush;
pub mod draw_event;
mod hot_reloading;
/// # CREDIT
/// A big part of the raycasting logic was stolen
/// from the [bevy_mod_raycast](https://github.com/aevyrie/bevy_mod_raycast) crate.
///
/// Since I only use a tiny part, which isn't the focus of the crate, I ported it into this file.
pub mod ray_cast;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RayCastPlugin)
            .add_plugin(
                ResourceInspectorPlugin::<ActiveBrush>::default()
                    .run_if(input_toggle_active(true, KeyCode::Escape)),
            )
            .insert_resource(ActiveBrush::new(Brushes::default()))
            .register_type::<ActiveBrush>()
            .add_event::<DrawEvent>()
            .add_system(draw_map)
            .add_system(notify_image_change);
    }
}
