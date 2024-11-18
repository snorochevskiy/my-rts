use bevy::{prelude::*, window::PrimaryWindow};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{util::projection::project_on_terrain, MyGroundPlane};

pub struct MyUnitsPlugin;

impl Plugin for MyUnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyGroundCoords>()
            .add_systems(Startup, setup_units)
            .add_systems(Update, cursor_to_ground_plane);
    }
}

#[derive(Resource, Default)]
struct MyGroundCoords {
    global: Vec3,
    local: Vec2,
}

#[derive(Component)]
pub struct MyUnit;

fn setup_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn(
        (
            SceneBundle {
                scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/units/Tank.glb")),
                ..default()
            },
            MyUnit
        )
    );
}

// Taken from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html#3d-games
fn cursor_to_ground_plane(
    mut commands: Commands,
    mut mycoords: ResMut<MyGroundCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>, // MyGameCamera
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
    asset_server: Res<AssetServer>
) {
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
    let (camera, camera_transform) = q_camera.single();

    let ground_transform = q_plane.single();

    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(global_cursor ) = project_on_terrain(cursor_position, &camera, &camera_transform, &ground_transform) else {
        return;
    };

    mycoords.global = global_cursor;
    eprintln!("\nGlobal cursor coords: {}/{}/{}", global_cursor.x, global_cursor.y, global_cursor.z);

    commands.spawn(
        (
            SceneBundle {
                scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/units/Tank.glb")),
                transform: Transform {
                    translation: Vec3 { x: global_cursor.x, y: global_cursor.y, z: global_cursor.z }, rotation: Quat::IDENTITY, scale: Vec3::ONE
                },
                ..default()
            },
            MyUnit,
        )
    );

    // to compute the local coordinates, we need the inverse of the plane's transform
    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

    // we can discard the Y coordinate, because it should always be zero (our point is supposed to be on the plane)
    mycoords.local = local_cursor.xz();
    //eprintln!("Local cursor coords: {}/{}", local_cursor.x, local_cursor.z);
}

