use bevy::prelude::{Entity, Plugin, Resource, Update};

use self::{
    brush::{Airbrush, Stencil},
    draw_event::{draw_map, DrawEvent},
    hot_reloading::notify_image_change,
    ray_cast::{RayCastPlugin, SelectedMap},
    save::{check_for_save_files, ImageSaver},
    ui::{run_ui, ActiveTool, Brush, BrushType},
};

use bevy_inspector_egui::{
    bevy_egui::{self, EguiSettings},
    DefaultInspectorConfigPlugin,
};

pub mod brush;
pub mod draw_event;
mod hot_reloading;
pub mod ray_cast;
pub mod save;
pub mod tools;
pub mod ui;

/// This plugin enables all functionality to run the editor
///
/// such as a simple gui, save functionality, live editing of maps
/// and simple ray casting.
#[derive(Default)]
pub struct EditorPlugin {
    pub scale_factor: f64,
}
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let egui_settings = EguiSettings {
            scale_factor: self.scale_factor,
            ..Default::default()
        };

        app.insert_resource(egui_settings)
            .add_plugins((
                RayCastPlugin,
                DefaultInspectorConfigPlugin,
                bevy_egui::EguiPlugin,
            ))
            .init_resource::<ActiveTool>()
            .init_resource::<ImageSaver>()
            .init_resource::<ActiveEditorChunk>()
            .init_resource::<SelectedMap>()
            .register_type::<BrushType>()
            .register_type::<Brush>()
            .register_type::<Stencil>()
            .register_type::<Airbrush>()
            .register_type::<ActiveTool>()
            .register_type::<SelectedMap>()
            .add_event::<DrawEvent>()
            .add_systems(
                Update,
                (run_ui, draw_map, check_for_save_files, notify_image_change),
            );
    }
}
/// Marker component for the entity that is currently edited
#[derive(Resource, Default)]
pub struct ActiveEditorChunk(pub Option<Entity>);
