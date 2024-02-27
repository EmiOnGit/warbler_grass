use bevy::{
    diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic},
    prelude::{Assets, Handle, InheritedVisibility, Plugin, Query, Res, Update},
    render::view::ViewVisibility,
};

use crate::dithering::DitheredBuffer;

/// A [`Plugin`] that logs the blades drawn in each frame.
///
/// # Example
/// ```rust
/// use bevy::diagnostic::LogDiagnosticsPlugin;
/// use bevy::prelude::App;
/// use warbler_grass::diagnostic::WarblerDiagnosticsPlugin;
///
/// App::new()
///     // add this plugin to log the values
///     .add_plugins(WarblerDiagnosticsPlugin)
///     // add bevys plugin to print all logged values to the terminal
///     .add_plugins(LogDiagnosticsPlugin::default());
/// ```
pub struct WarblerDiagnosticsPlugin;
impl Plugin for WarblerDiagnosticsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_diagnostic(
            // Adds the `Diagnostic` responsable for logging the blade count to the `Diagnostics`
            Diagnostic::new(Self::GRASS_BLADE_COUNT)
                .with_suffix(" blades")
                .with_max_history_length(20),
        )
        .add_systems(Update, Self::measure_blades);
    }
}
impl WarblerDiagnosticsPlugin {
    /// An id for the [`Diagnostic`] of the blade count.
    pub const GRASS_BLADE_COUNT: DiagnosticPath = DiagnosticPath::const_new("grass/blade_count");

    /// Calculates the amount of blades that are drawn this frame and logs them
    fn measure_blades(
        blades: Query<(
            &Handle<DitheredBuffer>,
            &InheritedVisibility,
            &ViewVisibility,
        )>,
        dither: Res<Assets<DitheredBuffer>>,
        mut diagnostics: Diagnostics,
    ) {
        // entities spawned with the WarblersBundle
        let count: u32 = blades
            .iter()
            // We are only interested in visible chunks
            .filter(|(_handle, inherited_visibility, view_visibility)| {
                inherited_visibility.get() && view_visibility.get()
            })
            .filter_map(|(handle, _visible, _view_visibility)| dither.get(handle))
            .map(|buffer| buffer.positions.len() as u32)
            .sum();

        diagnostics.add_measurement(&Self::GRASS_BLADE_COUNT, || count as f64);
    }
}
