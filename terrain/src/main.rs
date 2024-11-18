mod util;
mod terrain;
mod text_map;

use bevy::{
    prelude::*, render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    }
};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use text_map::parse;

const TERRAIN: &'static str = r#"
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 \ | 1 1 1 1 1 1 1 1 | | / 1 1 1 1 
    1 1 1 1 - 2 2 2 2 2 2 2 2 2 2 2 - 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 - 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 1 1 1 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 1 2 1 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 1 1 1 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 
    1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 
    1 1 1 1 - 2 2 2 2 2 2 2 2 2 2 2 - 1 1 1 1 
    1 1 1 1 / | 1 1 1 | | 1 1 1 1 | \ 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
"#;

#[derive(Component)]
struct CustomUV;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, init_configs_system)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn init_configs_system(
    mut fps: ResMut<FramepaceSettings>,
) {
    fps.limiter = Limiter::from_framerate(30.0);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let custom_texture_handle: Handle<Image> = asset_server.load("textures/mud_cracked_dry_03_diff_1k.png");

    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            metallic: 0.1,
            perceptual_roughness: 0.9,
            //alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        CustomUV,
    ));



    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 70.0, 50.0)).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            //shadow_depth_bias: 0.0,
            //shadow_normal_bias: 5.0,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 100.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
    ));
    ambient_light.brightness = 500.0;

    commands.spawn((
        Text::new("Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}

#[rustfmt::skip]
fn create_cube_mesh() -> Mesh {
    let tiles = parse(TERRAIN).unwrap();
    let (vertices, triangles, normals, uvs) = terrain::build_mesh(tiles);

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices,
    )
    .with_inserted_indices(Indices::U32(triangles))
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0,uvs)
    .with_generated_tangents().unwrap()
}
