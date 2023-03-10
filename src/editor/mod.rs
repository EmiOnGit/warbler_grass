use bevy::prelude::Plugin;

use self::{
    brush::ActiveBrush,
    draw_event::{draw_map, DrawEvent},
    hot_reloading::notify_image_change,
    ray_cast::{RayCastPlugin, SelectedMap},
};

use bevy_inspector_egui::{bevy_egui::EguiSettings, quick::ResourceInspectorPlugin};

use crate::editor::brush::Brushes;

pub mod brush;
pub mod draw_event;
mod hot_reloading;
/// # CREDIT
/// A big part of the raycasting logic was stolen
/// from the [bevy_mod_raycast](https://github.com/aevyrie/bevy_mod_raycast) crate.
///
/// Since I only use a tiny part, which isn't the focus of the crate, I ported it into this file.
pub mod ray_cast;
pub mod tools;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut egui_settings = EguiSettings::default();
        egui_settings.scale_factor = 2.;
        app.add_plugin(RayCastPlugin)
            .insert_resource(egui_settings)
            .add_plugin(ResourceInspectorPlugin::<ActiveBrush>::default())
            .add_plugin(ResourceInspectorPlugin::<SelectedMap>::default())
            .insert_resource(ActiveBrush::new(Brushes::default()))
            .init_resource::<SelectedMap>()
            .register_type::<ActiveBrush>()
            .register_type::<SelectedMap>()
            .add_event::<DrawEvent>()
            .add_system(draw_map)
            .add_system(notify_image_change);
    }
}
