use bevy::prelude::*;

pub struct MyDebugSpatialPlugin;

impl Plugin for MyDebugSpatialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DebugDrawPoint>()
            .add_systems(Update, draw_debug_points);
    }
}

#[derive(Event)]
pub struct DebugDrawPoint(pub Vec3);

#[derive(Component)]
struct DebugSpatial;

fn draw_debug_points(
    mut commands: Commands,
    old_points_q: Query<Entity, With<DebugSpatial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut reader: EventReader<DebugDrawPoint>,
) {
    if !reader.is_empty() {
        for old in old_points_q.iter() {
            commands.entity(old).despawn();
        }
    }

    let mut materials_iterator = DebugMaterialIterator::new(&mut materials);
    for DebugDrawPoint(e) in reader.read() {
        let debug_rect = meshes.add(Cuboid {half_size: Vec3 { x: 0.2, y: 0.2, z: 0.2 }});

        commands.spawn(
            (
                PbrBundle {
                    mesh: debug_rect.clone(),
                    material: materials_iterator.next().unwrap(),
                    transform: Transform::from_translation(*e),
                    ..default()
                },
                DebugSpatial
            )
        );

    }
}

const DEBUG_COLORS: &[Color] = &[
    Color::WHITE,
    Color::srgb(1.0, 0.0, 0.0),
    Color::srgb(0.0, 1.0, 0.0),
    Color::srgb(0.0, 0.0, 1.0),
];

struct DebugMaterialIterator<'a> {
    materials: &'a mut Assets<StandardMaterial>,
    next_color: usize,
}

impl DebugMaterialIterator<'_> {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> DebugMaterialIterator {
        DebugMaterialIterator {
            materials, next_color: 0
        }
    }
}

impl Iterator for DebugMaterialIterator<'_> {
    type Item = Handle<StandardMaterial>;

    fn next(&mut self) -> Option<Self::Item> {
        let meterial = self.materials.add(StandardMaterial {
            base_color: DEBUG_COLORS[self.next_color],
            ..default()
        });
        self.next_color = (self.next_color + 1) % DEBUG_COLORS.len();
        Some(meterial)
    }
}