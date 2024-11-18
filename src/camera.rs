use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_keyboard_controls);
    }
}

fn setup_camera(mut commands: Commands,) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Other(999),
            ..default()
        },
    ));
}

fn camera_keyboard_controls(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        let mut  movement = Vec3::ZERO;

            let mut delta_translation = Vec3::ZERO;
            if key_input.pressed(KeyCode::ArrowRight) || key_input.pressed(KeyCode::KeyD) {
                movement += Vec3::X;
            }
            if key_input.pressed(KeyCode::ArrowLeft) || key_input.pressed(KeyCode::KeyA) {
                movement += Vec3::NEG_X;
            }
            if key_input.pressed(KeyCode::ArrowUp) || key_input.pressed(KeyCode::KeyW) {
                movement += Vec3::NEG_Z;
            }
            if key_input.pressed(KeyCode::ArrowDown) || key_input.pressed(KeyCode::KeyS) {
                movement += Vec3::Z;
            }

            if movement != Vec3::ZERO {
                //Camera should move in X,Z only (horizontal) regardles of the camera rotation
                let mut rotation = transform.rotation;
                rotation.x = 0.0;
                rotation.z = 0.0;

                // The higher camera is, the faster it should move
                let speed = transform.translation.y * 2.0;

                delta_translation += (rotation * movement).normalize() * time.delta_seconds() * speed;

                transform.translation += delta_translation;
                pan_orbit.target_focus += delta_translation;
            }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}