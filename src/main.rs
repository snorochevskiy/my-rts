mod util;
mod debug;
mod camera;
mod selection;
mod units;

use bevy::prelude::*;
use bevy::{
    pbr::CascadeShadowConfigBuilder,
    render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor}
};
use bevy_framepace::{FramepaceSettings, Limiter};

use camera::MyCameraPlugin;
use debug::DebugSpatialPlugin;
use selection::{MySelectionPlugin, SelectionBoxCompleted};
use units::MyUnitsPlugin;

fn main() {
    App::new()
        .add_event::<SelectionBoxCompleted>()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(MySelectionPlugin)
        .add_plugins(MyUnitsPlugin)
        .add_plugins(DebugSpatialPlugin)
        .add_systems(Startup, init_configs_system)
        .add_systems(Startup, setup_scene)
        .run();
}



#[derive(Component)]
pub struct MyGroundPlane;

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

    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)), //.subdivisions(10)
                material: terrain_material_handle,
                ..default()
            },
            MyGroundPlane,
        )
    );
}
