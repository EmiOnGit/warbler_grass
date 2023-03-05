use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use warbler_grass::{editor::ray_cast::RayCamera, grass_spawner::GrassSpawner};

pub struct SimpleCamera;
impl Plugin for SimpleCamera {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_movement);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15., 55.0)
                .looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
            ..default()
        },
        RayCamera::default(),
    ));
}
fn camera_movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        let move_speed = 0.2;
        let rotate_speed = 0.02;
        let mut forward = transform.forward();
        forward.y = 0.;
        let right = transform.right();

        if input.pressed(KeyCode::W) {
            transform.translation += forward * move_speed;
        }
        if input.pressed(KeyCode::S) {
            transform.translation -= forward * move_speed;
        }
        if input.pressed(KeyCode::Q) {
            transform.rotate_y(rotate_speed);
        }
        if input.pressed(KeyCode::E) {
            transform.rotate_y(-rotate_speed);
        }
        if input.pressed(KeyCode::A) {
            transform.translation -= right * move_speed;
        }
        if input.pressed(KeyCode::D) {
            transform.translation += right * move_speed;
        }
    }
}
#[allow(dead_code)]
pub fn get_grass_grid() -> GrassSpawner {
    let positions = (0..10_000)
        .into_iter()
        .map(|i| Vec3::new((i / 100) as f32, 0., (i % 100) as f32) / 2.)
        .collect();
    GrassSpawner::new().with_positions(positions)
}

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_fps)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_system(diagnostic_system);
    }
}
pub fn setup_fps(mut diagnostics: ResMut<Diagnostics>) {
    diagnostics.add(Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS, "fps", 200));
}
pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, time: Res<Time>) {
    let delta_seconds = time.raw_delta_seconds_f64();
    if delta_seconds == 0.0 {
        return;
    }
    diagnostics.add_measurement(FrameTimeDiagnosticsPlugin::FPS, || 1.0 / delta_seconds);
}

// needed for rust-analyzer to be happy
#[allow(dead_code)]
fn main() {}
