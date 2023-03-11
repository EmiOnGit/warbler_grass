use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, FontId, RichText},
    inspector_options::std_options::NumberDisplay,
    prelude::*,
};

use super::{
    brush::{Airbrush, BrushBehavior, Stencil},
    ray_cast::SelectedMap,
    save::ImageSaver,
    tools::{Eraser, Filler},
};

pub fn run_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    let section_distance = 30.;
    egui::Window::new("Editor").show(egui_context.get_mut(), |ui| {
        ui.label(
            RichText::new("Active Tool")
                .font(FontId::proportional(20.0))
                .underline(),
        );

        bevy_inspector_egui::bevy_inspector::ui_for_resource::<ActiveTool>(world, ui);
        ui.add_space(section_distance);
        ui.separator();

        ui.push_id(1, |ui| {
            ui.label(
                RichText::new("Selected Map")
                    .font(FontId::proportional(20.0))
                    .underline(),
            );

            let selected_map = world.get_resource_mut::<SelectedMap>();
            if let Some(mut selected) = selected_map {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut *selected, SelectedMap::DensityMap, "Density map");
                    ui.radio_value(&mut *selected, SelectedMap::HeightMap, "Height map");
                    ui.radio_value(&mut *selected, SelectedMap::HeightsMap, "Heights map");
                });
            }
            ui.add_space(section_distance);
        });
        ui.separator();
        ui.push_id(2, |ui| {
            ui.label(
                RichText::new("Save Active Map")
                    .font(FontId::proportional(20.0))
                    .underline(),
            );
            if ui.button("save").clicked() {
                let saver = world.get_resource_mut::<ImageSaver>();
                if let Some(mut saver) = saver {
                    saver.path = rfd::FileDialog::new().save_file();
                    saver.set_changed();
                }
            }
            ui.add_space(section_distance);
        });
    });
}
/// The currently active tool in the editor
/// This can be directly choosen in the ui of the editor
/// Depending on the active tool, different things happen if you click on your map
#[derive(Resource, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub enum ActiveTool {
    Brush(Brush),
    Eraser,
    Filler,
}

impl Default for ActiveTool {
    fn default() -> Self {
        ActiveTool::Brush(Brush::default())
    }
}
impl ActiveTool {
    pub fn apply(&mut self, image: &mut Image, position: Vec2) {
        match self {
            ActiveTool::Brush(brush) => brush.draw(image, position),
            ActiveTool::Eraser => Eraser::erase(image),
            ActiveTool::Filler => Filler::fill(image),
        }
    }
}

/// The brush which can be used to draw on a
/// Can be configured in the editor ui
#[derive(InspectorOptions, Reflect, FromReflect, Clone)]
#[reflect(InspectorOptions, Default)]
pub struct Brush {
    /// Each [`BrushType`] has a implementation on how to draw
    brush: BrushType,
    /// Indicates the area of affect the [`Brush`] has
    /// The translation to the total area on the image is up to the implementation of the brush itself
    /// and doesn't have to be equal
    #[inspector(min = 2, max = 20 , display = NumberDisplay::Slider)]
    brush_size: u32,
    /// The strength of effect the [`Brush`] has
    /// The influence of this paramenter can vary depending on the internal implementation of the brush
    /// A negative value usually inverses the effect of the brush
    #[inspector(min = -30.0, max = 30.0, display = NumberDisplay::Slider)]
    strength: f32,
}
impl Default for Brush {
    fn default() -> Self {
        Brush {
            brush: BrushType::default(),
            brush_size: 10,
            strength: 10.,
        }
    }
}
/// The currently implemented Brushes.
/// They can be choosen from the ui in the editor
#[derive(Reflect, FromReflect, InspectorOptions, Clone)]
#[reflect(InspectorOptions, Default)]
pub enum BrushType {
    Stencil(Stencil),
    Airbrush(Airbrush),
}
impl Default for BrushType {
    fn default() -> Self {
        Self::Airbrush(Airbrush::default())
    }
}

impl Brush {
    fn draw(&self, image: &mut Image, position: Vec2) {
        match &self.brush {
            BrushType::Stencil(stencil) => {
                stencil.draw(image, position, self.brush_size, self.strength)
            }
            BrushType::Airbrush(air_brush) => {
                air_brush.draw(image, position, self.brush_size, self.strength)
            }
        }
    }
}
