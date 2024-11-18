use bevy::{pbr::NotShadowCaster, prelude::*, render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor}};

pub struct MyTerrainPlugin;

impl Plugin for MyTerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_terrain);
    }
}

#[derive(Component)]
pub struct MyGroundPlane;

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let texture_handle = asset_server.load_with_settings("textures/terrain/mud_cracked_dry_03_diff_1k.png",
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
    let normal_handle = asset_server.load_with_settings("textures/terrain/mud_cracked_dry_03_nor_gl_1k.png",
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
        normal_map_texture: Some(normal_handle),
        metallic: 0.1,
        perceptual_roughness: 0.9,
        alpha_mode: AlphaMode::Blend,
        //unlit: true,
        uv_transform: bevy::math::Affine2::from_scale(Vec2 {x: 10.0 , y: 10.0}),
        ..default()
    });

    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)), 
                material: terrain_material_handle,                
                ..default()
            },
            NotShadowCaster,
            MyGroundPlane,
        )
    );

}