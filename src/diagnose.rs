use bevy::{prelude::{Plugin, ResMut, Query, Handle, Assets, Res, ComputedVisibility}, diagnostic::{Diagnostics, Diagnostic, DiagnosticId}};

use crate::{dithering::DitheredBuffer, prelude::Grass};

/// A [`Plugin`] that logs the blades drawn in each frame.
/// 
/// If you want to simply log the values in the terminal,
/// you can also add the [`LogDiagnosticsPlugin`] to your app
/// 
/// # Example
/// ```rust
/// use bevy::diagnostic::LogDiagnosticsPlugin;
/// use bevy::prelude::App;
/// use warbler_grass::diagnose::WarblerDiagnosticsPlugin;
/// 
/// App::new()
///     // add this plugin to log the values
///     .add_plugin(WarblerDiagnosticsPlugin)
///     // add bevys plugin to print all logged values to the terminal
///     .add_plugin(LogDiagnosticsPlugin::default());
/// ```
pub struct WarblerDiagnosticsPlugin;
impl Plugin for WarblerDiagnosticsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_startup_system(Self::setup_blade_count)
        .add_system(Self::measure_blades);
    }
}
impl WarblerDiagnosticsPlugin {
    /// An id for the [`Diagnostic`] of the blade count
    pub const GRASS_BLADE_COUNT: DiagnosticId =
    DiagnosticId::from_u128(11920430925311532474622109399490581929);

    /// Adds the [`Diagnostic`] responsable for logging the blade count to the [`Diagnostics`]
    fn setup_blade_count(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::GRASS_BLADE_COUNT, "grass blade count", 20).with_suffix(" blades"));
    
    }

    /// Calculates the amount of blades that are drawn this frame and logs them
    fn measure_blades (
        blades: Query<(&Handle<DitheredBuffer>, &ComputedVisibility)>,
        explicit_blades: Query<(&Grass, &ComputedVisibility)>,
        dither: Res<Assets<DitheredBuffer>>,
        mut diagnostics: ResMut<Diagnostics>,
    ) {
        // entities spawned with the WarblersBundle
        let count: u32 = blades.iter()
            // We are only interested in visible chunks
            .filter(|(_handle, visible)| visible.is_visible())
            .filter_map(|(handle,_visible)| dither.get(handle)).map(|buffer| buffer.positions.len() as u32)
            .sum();

        // entities spawned with the WarblersExplicitBundle
        let count_explicit: u32 = explicit_blades.iter()
            .filter(|(_grass, visible)| visible.is_visible())
            .map(|(grass,_visible)| grass.positions.len() as u32)
            .sum();

        diagnostics.add_measurement(Self::GRASS_BLADE_COUNT, || count as f64 + count_explicit as f64);

    }
}

