use bevy::{prelude::*, window::PrimaryWindow};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{debug::DebugDrawPoint, units::MyUnit, util::{point_2d::Trapez, projection::project_on_terrain}, MyGroundPlane};

pub struct MySelectionPlugin;

impl Plugin for MySelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, select_box)
            .add_systems(Update, perform_selection);
    }
}

#[derive(Component)]
struct SelectionBoxInProcess {
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Debug, Clone)]
pub struct SelectionBoxCompleted(pub Trapez);

fn select_box(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut select_box_query: Query<(Entity, &mut Style, &SelectionBoxInProcess)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>, // MyGameCamera
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
    mut producer: EventWriter<SelectionBoxCompleted>
) {
    if let Ok((entity, mut style, center)) = select_box_query.get_single_mut() {
        if mouse_input.just_released(MouseButton::Left) {
            commands.entity(entity).despawn();

            let (camera, camera_transform) = q_camera.single();
            let ground_transform = q_plane.single();

            let Some(top_left) = project_on_terrain(Vec2::new(style.left.get_px(), style.top.get_px()), &camera, &camera_transform, &ground_transform) else {
                return;
            };
            let Some(top_right) = project_on_terrain(Vec2::new(style.left.get_px() + style.width.get_px(), style.top.get_px()), &camera, &camera_transform, &ground_transform) else {
                return;
            };
            let Some(bottom_right) = project_on_terrain(Vec2::new(style.left.get_px() + style.width.get_px(), style.top.get_px() + style.height.get_px()), &camera, &camera_transform, &ground_transform) else {
                return;
            };
            let Some(bottom_left) = project_on_terrain(Vec2::new(style.left.get_px(), style.top.get_px() + style.height.get_px()), &camera, &camera_transform, &ground_transform) else {
                return;
            };

            producer.send(SelectionBoxCompleted (Trapez { top_left, top_right, bottom_left, bottom_right}));
            return;
        }
        let window = q_window.single();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        if center.y < cursor_position.y {
            style.top = Val::Px(center.y);
            style.height = Val::Px(cursor_position.y - style.top.get_px());
        } else if center.y > cursor_position.y {
            style.top = Val::Px(cursor_position.y);
            style.height = Val::Px(center.y - cursor_position.y);
        }
        if center.x < cursor_position.x {
            style.left = Val::Px(center.x);
            style.width = Val::Px(cursor_position.x - style.left.get_px());
        } else if center.x > cursor_position.x {
            style.left = Val::Px(cursor_position.x);
            style.width = Val::Px(center.x - cursor_position.x);
        }
        
        
    } else if mouse_input.just_pressed(MouseButton::Left) {
        let window = q_window.single();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
    
        // Block selection for units
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(0.0),
                    height: Val::Px(0.0),
                    left: Val::Px(cursor_position.x),
                    top: Val::Px(cursor_position.y),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::Srgba(Srgba { red: 0.21960784, green: 0.7411765, blue: 0.972549, alpha: 0.2 })),
                ..Default::default()
            },
            SelectionBoxInProcess { x: cursor_position.x, y: cursor_position.y},
        ));
    }
}

fn perform_selection(
    units_q: Query<&Transform, With<MyUnit>>,
    mut reader: EventReader<SelectionBoxCompleted>,
    mut debug_point_writer: EventWriter<DebugDrawPoint>,
) {
    for SelectionBoxCompleted(e) in reader.read() {
        println!("\nSelection box: {} - {}", e.top_left, e.bottom_right);
        for u in units_q.iter() {
            if e.contains(u.translation) {
                println!("Unit {} is in selection: TRUE",  u.translation);
            } else {
                println!("Unit {} is in selection: FALSE",  u.translation);
            }

            debug_point_writer.send(DebugDrawPoint(e.top_left));
            debug_point_writer.send(DebugDrawPoint(e.top_right));
            debug_point_writer.send(DebugDrawPoint(e.bottom_right));
            debug_point_writer.send(DebugDrawPoint(e.bottom_left));
        }
    }
}

trait ValPx {
    fn get_px(&self) -> f32;
}

impl ValPx for Val {
    fn get_px(&self) -> f32 {
        match self {
            Val::Px(v) => *v,
            _ => unreachable!("Shouldn't be here"),
        }
    }
}
