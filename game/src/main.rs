mod util;
mod debug;
mod camera;
mod selection;
mod units;
mod terrain;

use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_framepace::{FramepaceSettings, Limiter};

use camera::MyCameraPlugin;
use debug::MyDebugSpatialPlugin;
use selection::{MySelectionPlugin, SelectionBoxCompleted};
use terrain::MyTerrainPlugin;
use units::MyUnitsPlugin;

fn main() {
    App::new()
        .add_event::<SelectionBoxCompleted>()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::pbr::wireframe::WireframePlugin)
        .insert_resource(bevy::pbr::wireframe::WireframeConfig {
            global: true,
            default_color: bevy::color::palettes::css::WHITE.into(),
        })
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(MyTerrainPlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(MySelectionPlugin)
        .add_plugins(MyUnitsPlugin)
        .add_plugins(MyDebugSpatialPlugin)
        .add_systems(Startup, init_configs_system)
        .add_systems(Startup, setup_scene)
        .run();
}



fn init_configs_system(
    mut fps: ResMut<FramepaceSettings>,
) {
    fps.limiter = Limiter::from_framerate(30.0);
}

fn setup_scene(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            //illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 100.00).looking_at(Vec3::new(-10.0, -10.0, 0.0), Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            num_cascades: 1,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });

}
