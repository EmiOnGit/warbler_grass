use bevy::prelude::*;

/// Used in the example to spawn a simple camera which is moves with qweasd keys
pub struct SimpleCamera;
impl Plugin for SimpleCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 15., -20.0)
            .looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        ..default()
    },));
}
pub fn camera_movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        let move_speed = 0.6;
        let rotate_speed = 0.03;
        let mut forward = transform.forward();
        forward.y = 0.;
        let right = transform.right();
        let up = transform.up();

        if input.pressed(KeyCode::W) {
            transform.translation += forward * move_speed;
        }
        if input.pressed(KeyCode::S) {
            transform.translation -= forward * move_speed;
        }
        if input.pressed(KeyCode::Space) {
            transform.translation += up * move_speed;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            transform.translation -= up * move_speed;
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

// needed for rust-analyzer to be happy
#[allow(dead_code)]
fn main() {}
