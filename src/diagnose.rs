use bevy::{prelude::{Plugin, ResMut, Query, Handle, Assets, Res, ComputedVisibility}, diagnostic::{LogDiagnosticsPlugin, Diagnostics, Diagnostic, DiagnosticId}};

use crate::dithering::DitheredBuffer;


pub struct WarblerDiagnosePlugin;
impl Plugin for WarblerDiagnosePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_startup_system(Self::setup_blade_count)
        .add_system(Self::measure_blades)
        .add_plugin(LogDiagnosticsPlugin::default());
    }
}
impl WarblerDiagnosePlugin {
    pub const GRASS_BLADE_COUNT: DiagnosticId =
    DiagnosticId::from_u128(81920430925311532474622109399490581929);
    
    fn setup_blade_count(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::GRASS_BLADE_COUNT, "grass blade count", 20).with_suffix(" blades"));
    
    }
    fn measure_blades (
        blades: Query<(&Handle<DitheredBuffer>, &ComputedVisibility)>,
        dither: Res<Assets<DitheredBuffer>>,
        mut diagnostics: ResMut<Diagnostics>,
    ) {
        let sum: u32 = blades.iter()
            .filter(|(_handle, visible)| visible.is_visible())
            .filter_map(|(handle,_visible)| dither.get(handle)).map(|buffer| buffer.positions.len() as u32)
            .sum();
        diagnostics.add_measurement(Self::GRASS_BLADE_COUNT, || sum as f64);

    }
}

