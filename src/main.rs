use bevy::prelude::*;
use bevy::{
    pbr::CascadeShadowConfigBuilder,
    render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor}
};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};


#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, init_configs_system)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, keyboard_controls)
        .run();
}

fn init_configs_system(
    mut fps: ResMut<FramepaceSettings>,
) {
    fps.limiter = Limiter::from_framerate(30.0);
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.1, 1.0, 0.1).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    let texture_handle = asset_server.load_with_settings("textures/terrain/grass1-albedo3.png",
    |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            }),
            ..default()
        }
    });
    let terrain_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        uv_transform: bevy::math::Affine2::from_scale(Vec2 {x: 10.0 , y: 10.0}),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)), //.subdivisions(10)
        material: terrain_material_handle, //materials.add(StandardMaterial::from_color(Color::WHITE)),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/units/Tank.glb")),
        ..default()
    });
}

fn keyboard_controls(
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