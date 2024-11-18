use bevy::math::{Mat3, Vec3};

use crate::util::MatrixHelper;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Left,
    Top,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Side {
    pub fn ramp_normal(&self) -> [f32; 3] {
        match self {
            Side::Left =>        [-0.7, 0.7, 0.0],
            Side::Top =>         [0.0, 0.7, -0.7],
            Side::Right =>       [0.7, 0.7, 0.0],
            Side::Bottom =>      [0.0, 0.7, 0.7],
            Side::TopLeft =>     [-0.7, 0.7, -0.7],
            Side::TopRight =>    [0.7, 0.7, -0.7],
            Side::BottomLeft =>  [-0.7, 0.7, 0.7],
            Side::BottomRight => [0.7, 0.7, 0.7],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Plain {
    pub level: f32,
    pub cliffs: Vec<Side>
}

#[derive(Debug, PartialEq)]
pub struct Ramp {
    pub bottom_level: f32,
    pub top_level: f32,
    pub bottom_side: Side,
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    Plain(Plain),
    Ramp(Ramp),
}

impl Tile {
    pub fn as_ramp(&self) -> Option<&Ramp> {
        match self {
            Tile::Ramp(ramp) => Some(ramp),
            _ => None,
        }
    }

    pub fn as_plain(&self) -> Option<&Plain> {
        match self {
            Tile::Plain(plain) => Some(plain),
            _ => None,
        }
    }
}

pub fn build_mesh(tiles: Vec<Vec<Tile>>) -> (Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let quad_size = 5.0;
    let quad_height = 5.0;

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<u32> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let vertical_tiles = tiles.len();
    let horizontal_tiles = tiles[0].len();

    let half_y = vertical_tiles as f32 / 2.0 - 0.5;
    let half_x = horizontal_tiles as f32 / 2.0 - 0.5;

    for (row_ind, row) in tiles.iter().enumerate() {
        for (col_ind, col) in row.iter().enumerate() {
            let row_ind = row_ind as i32;
            let col_ind = col_ind as i32;
            let shift_x = col_ind as f32 - half_x;
            let shift_z = row_ind as f32 - half_y;

            match col {
                Tile::Plain(Plain { level, cliffs }) => {
                    let first_vert_ind = vertices.len();

                    let plain_vertices = [
                        [-quad_size / 2.0, *level * quad_height, quad_size / 2.0],
                        [-quad_size / 2.0, *level * quad_height, -quad_size / 2.0],
                        [quad_size / 2.0,  *level * quad_height, -quad_size / 2.0],
                        [quad_size / 2.0,  *level * quad_height, quad_size / 2.0],
                    ];
                    let shifted_plain_vertices = shift_vertices(&plain_vertices, shift_x * quad_size, shift_z * quad_size);
                    vertices.extend_from_slice(&shifted_plain_vertices);

                    let mut plain_normals = [[0.0, 1.0, 0.0]; 4];
                    match tiles.cell(row_ind + 1, col_ind).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↓
                        Some(Side::Bottom) => {
                            plain_normals[0] = Side::Bottom.ramp_normal();
                            plain_normals[3] = Side::Bottom.ramp_normal();
                        },
                        Some(Side::Top) => {
                            plain_normals[0] = Side::Top.ramp_normal();
                            plain_normals[3] = Side::Top.ramp_normal();
                        },
                        Some(Side::Left) => plain_normals[0] = Side::Left.ramp_normal(),
                        Some(Side::Right) => plain_normals[3] = Side::Right.ramp_normal(),
                        Some(Side::TopLeft) => plain_normals[0] = Side::TopLeft.ramp_normal(),
                        Some(Side::TopRight) => plain_normals[3] = Side::TopRight.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind - 1, col_ind).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↑
                        Some(Side::Bottom) => {
                            plain_normals[1] = Side::Bottom.ramp_normal();
                            plain_normals[2] = Side::Bottom.ramp_normal();
                        },
                        Some(Side::Top) => {
                            plain_normals[1] = Side::Top.ramp_normal();
                            plain_normals[2] = Side::Top.ramp_normal();
                        },
                        Some(Side::Left) => plain_normals[1] = Side::Left.ramp_normal(),
                        Some(Side::Right) => plain_normals[2] = Side::Right.ramp_normal(),
                        Some(Side::BottomLeft) => plain_normals[1] = Side::BottomLeft.ramp_normal(),
                        Some(Side::BottomRight) => plain_normals[2] = Side::BottomRight.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind, col_ind - 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ←
                        Some(Side::Left) => {
                            plain_normals[0] = Side::Left.ramp_normal();
                            plain_normals[1] = Side::Left.ramp_normal();
                        },
                        Some(Side::Right) => {
                            plain_normals[0] = Side::Right.ramp_normal();
                            plain_normals[1] = Side::Right.ramp_normal();
                        },
                        Some(Side::Top) => plain_normals[1] = Side::Top.ramp_normal(),
                        Some(Side::Bottom) => plain_normals[0] = Side::Bottom.ramp_normal(),
                        Some(Side::TopRight) => plain_normals[1] = Side::TopRight.ramp_normal(),
                        Some(Side::BottomRight) => plain_normals[0] = Side::BottomRight.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind, col_ind + 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // →
                        Some(Side::Right) => {
                            plain_normals[2] = Side::Right.ramp_normal();
                            plain_normals[3] = Side::Right.ramp_normal();
                        },
                        Some(Side::Left) => {
                            plain_normals[2] = Side::Left.ramp_normal();
                            plain_normals[3] = Side::Left.ramp_normal();
                        },
                        Some(Side::Top) => plain_normals[2] = Side::Top.ramp_normal(),
                        Some(Side::Bottom) => plain_normals[3] = Side::Bottom.ramp_normal(),
                        Some(Side::TopLeft) => plain_normals[2] = Side::TopLeft.ramp_normal(),
                        Some(Side::BottomLeft) => plain_normals[3] = Side::BottomLeft.ramp_normal(),
                        _ => (),
                    }

                    match tiles.cell(row_ind + 1, col_ind - 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↙
                        Some(side) if !is_corner_ramp(&side) || side == Side::TopRight => plain_normals[0] = side.ramp_normal(),
                        Some(Side::BottomLeft) => plain_normals[0] = Side::BottomLeft.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind - 1, col_ind - 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↖
                        Some(side) if !is_corner_ramp(&side) || side == Side::BottomRight => plain_normals[1] = side.ramp_normal(),
                        Some(Side::TopLeft) => plain_normals[1] = Side::TopLeft.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind - 1, col_ind + 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↗
                        Some(side) if !is_corner_ramp(&side) || side == Side::BottomLeft => plain_normals[2] = side.ramp_normal(),
                        Some(Side::TopRight) => plain_normals[2] = Side::TopRight.ramp_normal(),
                        _ => (),
                    }
                    match tiles.cell(row_ind + 1, col_ind + 1).and_then(|t| t.as_ramp().map(|r|r.bottom_side)) { // ↘
                        Some(side) if !is_corner_ramp(&side) || side == Side::TopLeft => plain_normals[3] = side.ramp_normal(),
                        Some(Side::BottomRight) => plain_normals[3] = Side::BottomRight.ramp_normal(),
                        _ => (),
                    }

                    normals.extend_from_slice(&plain_normals);

                    uvs.extend_from_slice(&[[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);

                    triangles.push(first_vert_ind as u32);
                    triangles.push(first_vert_ind as u32 + 2);
                    triangles.push(first_vert_ind as u32 + 1);
                    triangles.push(first_vert_ind as u32);
                    triangles.push(first_vert_ind as u32 + 3);
                    triangles.push(first_vert_ind as u32 + 2);

                    for cliff in cliffs {
                        let first_vert_ind = vertices.len();

                        let cliff_vertices = [
                            [-quad_size / 2.0, (*level - 1.0) * quad_height, quad_size / 2.0],
                            [-quad_size / 2.0, *level * quad_height,         quad_size / 2.0],
                            [quad_size / 2.0,  *level * quad_height,         quad_size / 2.0],
                            [quad_size / 2.0,  (*level - 1.0) * quad_height, quad_size / 2.0],
                        ];
                        let rotated_vertices = rotate_vertices(&cliff_vertices, cliff);
                        let shifted_vertices = shift_vertices(&rotated_vertices, shift_x * quad_size, shift_z * quad_size);

                        vertices.extend_from_slice(&shifted_vertices);
                        normals.extend_from_slice(&rotate_vertices(&[[0.0, 0.0, 1.0]; 4], cliff));
                        uvs.extend_from_slice(&[[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);

                        triangles.push(first_vert_ind as u32);
                        triangles.push(first_vert_ind as u32 + 2);
                        triangles.push(first_vert_ind as u32 + 1);
                        triangles.push(first_vert_ind as u32);
                        triangles.push(first_vert_ind as u32 + 3);
                        triangles.push(first_vert_ind as u32 + 2);
                    }
                },
                Tile::Ramp(Ramp { bottom_level, top_level, bottom_side}) => {
                    let first_vert_ind = vertices.len();

                    let ramp_vertices = if is_corner_ramp(bottom_side) {
                        [
                            [-quad_size / 2.0, *bottom_level * quad_height, quad_size / 2.0],
                            [-quad_size / 2.0, *bottom_level * quad_height, -quad_size / 2.0],
                            [quad_size / 2.0,  *top_level * quad_height,    -quad_size / 2.0],
                            [quad_size / 2.0,  *bottom_level * quad_height, quad_size / 2.0],
                        ]
                    } else {
                        [
                            [-quad_size / 2.0, *bottom_level * quad_height, quad_size / 2.0],
                            [-quad_size / 2.0, *top_level * quad_height,    -quad_size / 2.0],
                            [quad_size / 2.0,  *top_level * quad_height,    -quad_size / 2.0],
                            [quad_size / 2.0,  *bottom_level * quad_height, quad_size / 2.0],
                        ]
                    };
                    let left_neighbor = tiles.cell_relative(row_ind, col_ind, ramp_left_shift(*bottom_side));
                    let right_neighbor = tiles.cell_relative(row_ind, col_ind, ramp_right_shift(*bottom_side));
                    let ramp_normals = if is_corner_ramp(bottom_side) {
                        [[-0.7, 0.7, 0.7], [-0.7, 0.7, 0.0], [-0.7, 0.7, 0.7], [0.0, 0.7, 0.7]]
                    } else {
                        let top_left_norm = match left_neighbor {
                            Some(Tile::Ramp(ramp)) if is_corner_ramp(&ramp.bottom_side) => [-0.7, 0.7, 0.7],
                            _ => [0.0, 0.7, 0.7],
                        };
                        let top_right_norm = match right_neighbor {
                            Some(Tile::Ramp(ramp)) if is_corner_ramp(&ramp.bottom_side) => [0.7, 0.7, 0.7],
                            _ => [0.0, 0.7, 0.7],
                        };
                        [[0.0, 0.7, 0.7], top_left_norm, top_right_norm, [0.0, 0.7, 0.7]]
                    };
                    let rotated_vertices = rotate_vertices(&ramp_vertices, bottom_side);
                    let shifted_vertices = shift_vertices(&rotated_vertices, shift_x * quad_size, shift_z * quad_size);

                    vertices.extend_from_slice(&shifted_vertices);
                    normals.extend_from_slice(&rotate_vertices(&ramp_normals, bottom_side));
                    uvs.extend_from_slice(&[[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);

                    triangles.push(first_vert_ind as u32);
                    triangles.push(first_vert_ind as u32 + 2);
                    triangles.push(first_vert_ind as u32 + 1);
                    triangles.push(first_vert_ind as u32);
                    triangles.push(first_vert_ind as u32 + 3);
                    triangles.push(first_vert_ind as u32 + 2);

                    if !is_corner_ramp(bottom_side) && left_neighbor.and_then(|t|t.as_plain()).is_some() {
                        let cliff_vertices = [
                            [-quad_size / 2.0, *bottom_level * quad_height, -quad_size / 2.0],
                            [-quad_size / 2.0, *top_level * quad_height,    -quad_size / 2.0],
                            [-quad_size / 2.0, *bottom_level * quad_height, quad_size / 2.0],
                        ];
                        let rotated_vertices = rotate_vertices(&cliff_vertices, bottom_side);
                        let shifted_vertices = shift_vertices(&rotated_vertices, shift_x * quad_size, shift_z * quad_size);

                        vertices.extend_from_slice(&shifted_vertices);
                        normals.extend_from_slice(&rotate_vertices(&[[-1.0, 0.0, 0.0];3], bottom_side));
                        uvs.extend_from_slice(&[[0.0, 1.0], [0.0, 0.0], [1.0, 1.0]]);

                        triangles.push(vertices.len() as u32 - 3);
                        triangles.push(vertices.len() as u32 - 1);
                        triangles.push(vertices.len() as u32 - 2);
                    }

                    if !is_corner_ramp(bottom_side) && right_neighbor.and_then(|t|t.as_plain()).is_some() {
                        let cliff_vertices = [
                            [quad_size / 2.0, *bottom_level * quad_height, quad_size / 2.0],
                            [quad_size / 2.0, *top_level * quad_height,    -quad_size / 2.0],
                            [quad_size / 2.0, *bottom_level * quad_height, -quad_size / 2.0],
                        ];
                        let rotated_vertices = rotate_vertices(&cliff_vertices, bottom_side);
                        let shifted_vertices = shift_vertices(&rotated_vertices, shift_x * quad_size, shift_z * quad_size);

                        vertices.extend_from_slice(&shifted_vertices);
                        normals.extend_from_slice(&rotate_vertices(&[[1.0, 0.0, 0.0];3], bottom_side));
                        uvs.extend_from_slice(&[[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]);

                        triangles.push(vertices.len() as u32 - 3);
                        triangles.push(vertices.len() as u32 - 1);
                        triangles.push(vertices.len() as u32 - 2);
                    }

                },
            }
        }
    }
    (vertices, triangles, normals, uvs)
}

fn rotate_vertices(points: &[[f32; 3]], side: &Side) -> Vec<[f32; 3]> {
    let angle = match side {
        Side::Left => std::f32::consts::PI + std::f32::consts::FRAC_PI_2,
        Side::Top => std::f32::consts::PI,
        Side::Right => std::f32::consts::FRAC_PI_2,
        Side::Bottom => 0.0,
        Side::TopLeft => std::f32::consts::PI + std::f32::consts::FRAC_PI_2,
        Side::TopRight => std::f32::consts::PI,
        Side::BottomLeft => 0.0,
        Side::BottomRight => std::f32::consts::FRAC_PI_2,
    };

    let rotation = Mat3::from_rotation_y(angle);

    points.iter()
        .map(|p| {
            let v3 = Vec3::from(*p);
            let Vec3 { x, y, z } = rotation * v3;
            [x, y, z]
        })
        .collect()
}

fn shift_vertices(points: &[[f32; 3]], x_shift: f32, z_shift: f32) -> Vec<[f32; 3]> {
    points.iter()
        .map(|&[x, y, z]| [x + x_shift, y, z + z_shift])
        .collect()
}

fn is_corner_ramp(side: &Side) -> bool {
    match side {
        Side::Left => false,
        Side::Top => false,
        Side::Right => false,
        Side::Bottom => false,

        Side::TopLeft => true,
        Side::TopRight => true,
        Side::BottomLeft => true,
        Side::BottomRight => true,
    }
}

pub fn ramp_left_shift(ramp_side: Side) -> (i32, i32) {
    match ramp_side {
        Side::Left => (-1, 0),
        Side::Top => (0, 1),
        Side::Right => (1, 0),
        Side::Bottom => (0, -1),
        Side::TopLeft => (0, 1),
        Side::TopRight => (1, 0),
        Side::BottomLeft => (-1, 0),
        Side::BottomRight => (0, -1),
    }
}

pub fn ramp_right_shift(ramp_side: Side) -> (i32, i32) {
    match ramp_side {
        Side::Left => (1, 0),
        Side::Top => (0, -1),
        Side::Right => (-1, 0),
        Side::Bottom => (0, 1),
        Side::TopLeft => (1, 0),
        Side::TopRight => (0, -1),
        Side::BottomLeft => (0, 1),
        Side::BottomRight => (-1, 0),
    }
}

