use bevy::prelude::*;

/// Finds clobal coordinates of cursor position on the plane that corresponds to the terrain.
/// 
/// ## Args:
/// * `cursor_position` - cursor position on the window
/// * `camera` - camera object
/// * `camera_transform` - camera global transformation
/// * `ground_transform` - terrain global transformation
/// 
/// ## Usage example
/// ```
/// fn my_system(
///     q_window: Query<&Window, With<PrimaryWindow>>,
///     q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>, // MyGameCamera
///     q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
/// ) {
///     if !mouse_input.just_pressed(MouseButton::Right) {
///         return;
///     }
///     let (camera, camera_transform) = q_camera.single();
/// 
///     let ground_transform = q_plane.single();
/// 
///     let window = q_window.single();
/// 
///     // check if the cursor is inside the window and get its position
///     let Some(cursor_position) = window.cursor_position() else {
///         return;
///     };
///     let Some(point) = project_on_terrain(&cursor_position, &camera, &camera_transform, &ground_transform);
/// ```
pub fn project_on_terrain(
    cursor_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    ground_transform: &GlobalTransform,
) -> Option<Vec3> {
    let plane_origin = ground_transform.translation();
    let plane = InfinitePlane3d {
        normal: ground_transform.up(),
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return None;
    };

    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return None;
    };

    let global_cursor = ray.get_point(distance);

    Some(global_cursor)
}