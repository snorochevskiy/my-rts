use bevy::{prelude::*, window::PrimaryWindow};
use bevy_panorbit_camera::PanOrbitCamera;
use parry2d::bounding_volume::BoundingSphere;

use crate::{terrain::MyGroundPlane, util::projection::project_on_terrain};

pub struct MyUnitsPlugin;

impl Plugin for MyUnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyGroundCoords>()
            .add_systems(Startup, setup_units)
            .add_systems(Update, spawn_tank)
            .add_systems(Update, send_selected_units)
            .add_systems(Update, move_units);
    }
}

#[derive(Resource, Default, Debug)]
pub struct SelectedUnits {
    pub unit_entities: Vec<Entity>,
}

#[derive(Resource, Default)]
struct MyGroundCoords {
    global: Vec3,
    local: Vec2,
}

#[derive(Component, Default)]
pub struct MovableUnit {
    pub half_size: f32,
    pub speed: f32,
    pub destination: Option<Vec3>,
}

fn setup_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn(
        (
            SceneBundle {
                scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/units/T72.glb")),
                ..default()
            },
            MovableUnit { half_size: 4.0, speed: 5.0, destination: None }
        )
    );
}

// Taken from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html#3d-games
fn spawn_tank(
    mut commands: Commands,
    mut mycoords: ResMut<MyGroundCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>, // MyGameCamera
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
    asset_server: Res<AssetServer>
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
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
                scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/units/T72.glb")),
                transform: Transform {
                    translation: Vec3 { x: global_cursor.x, y: global_cursor.y, z: global_cursor.z }, rotation: Quat::IDENTITY, scale: Vec3::ONE
                },
                ..default()
            },
            MovableUnit { half_size: 4.0, speed: 5.0, destination: None },
        )
    );

    // to compute the local coordinates, we need the inverse of the plane's transform
    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

    // we can discard the Y coordinate, because it should always be zero (our point is supposed to be on the plane)
    mycoords.local = local_cursor.xz();
    //eprintln!("Local cursor coords: {}/{}", local_cursor.x, local_cursor.z);
}


fn send_selected_units(
    selected_units: ResMut<SelectedUnits>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
    mut units_q: Query<(&mut MovableUnit, Entity)>,
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

    // Move units to point
    for (mut unit, entity) in units_q.iter_mut() {
        if selected_units.unit_entities.contains(&entity) {
            unit.destination = Some(global_cursor);
        }
    }
}

fn move_units(mut units_q: Query<(&mut Transform, &mut MovableUnit)>, time: Res<Time>) {
    for (mut tr, mut movable) in units_q.iter_mut() {
        if let Some(moving_destination) = movable.destination {

            let desired_rotation = tr.looking_at(moving_destination, Vec3::Y);
            let lerp = tr.rotation.lerp(desired_rotation.rotation, 2.0 * time.delta_seconds());
            tr.rotation = lerp;

            tr.translation = tr.translation.move_towards(moving_destination, 5.0 * time.delta_seconds());
            if tr.translation.distance(moving_destination) < 0.1 {
                movable.destination = None;
            }
        }
    }
}
