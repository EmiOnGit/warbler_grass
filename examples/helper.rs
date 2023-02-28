use bevy::prelude::*;
use warbler_grass::grass_spawner::GrassSpawner;

pub struct SimpleCamera;
impl Plugin for SimpleCamera {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_movement);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 8., -5.0).looking_at(Vec3::new(0., 5., 0.), Vec3::Y),
        ..default()
    });
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
    let positions = (0..10000)
        .into_iter()
        .map(|i| Vec3::new((i / 100) as f32, 0., (i % 100) as f32) / 2.)
        .collect();
    GrassSpawner::new().with_positions(positions)
}

// needed for rust-analyzer to be happy
#[allow(dead_code)]
fn main() {}
