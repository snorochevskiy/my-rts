use bevy::math::Vec3;

#[derive(Debug, Clone)]
pub struct Trapez {
    pub top_left: Vec3,
    pub top_right: Vec3,
    pub bottom_left: Vec3,
    pub bottom_right: Vec3
}

impl Trapez {
    pub fn normal_rotation(&self) -> Trapez {
        if self.top_left.x <= self.top_right.x && self.top_left.z <= self.bottom_left.z {
            self.clone()
        } else if self.top_right.x <= self.bottom_right.x && self.top_right.z <= self.top_left.z {
            // turn left 1
            Trapez { top_left: self.top_right, top_right: self.bottom_right, bottom_left: self.top_left, bottom_right: self.bottom_left }
        } else if self.bottom_right.x <= self.bottom_left.x && self.bottom_right.z <= self.top_right.z {
            // turn left 2
            Trapez { top_left: self.bottom_right, top_right: self.bottom_left, bottom_left: self.top_right, bottom_right: self.top_left }
        } else {
            // turn right 1
            Trapez { top_left: self.bottom_left, top_right: self.top_left, bottom_left: self.bottom_right, bottom_right: self.top_right }
        }
    }

    pub fn contains(&self, point: Vec3) -> bool {
        let n = self.normal_rotation();

        let is_below_upper_line = is_point_below_the_line(point, n.top_left, n.top_right);
        let is_above_bottom_line = is_point_above_the_line(point, n.bottom_left, n.bottom_right);
        let is_right_to_left_line = is_point_right_to_the_line(point, n.top_left, n.bottom_left);
        let is_left_to_right_line = is_point_left_to_the_line(point, n.top_right, n.bottom_right);

        is_below_upper_line && is_above_bottom_line && is_right_to_left_line && is_left_to_right_line
    }
}

fn calc_line_z(x: f32, p1: Vec3, p2: Vec3) -> f32 {
    (((x - p1.x) * (p2.z - p1.z)) / (p2.x - p1.x)) + p1.z
}

fn calc_line_x(z: f32, p1: Vec3, p2: Vec3) -> f32 {
    ((z - p1.z) * (p2.x - p1.x) / (p2.z - p1.z)) + p1.x
}

pub fn is_point_left_to_the_line(point: Vec3, p1: Vec3, p2: Vec3) -> bool {
    let x_on_the_line = calc_line_x(point.z, p1, p2);
    point.x < x_on_the_line
}

pub fn is_point_right_to_the_line(point: Vec3, p1: Vec3, p2: Vec3) -> bool {
    let x_on_the_line = calc_line_x(point.z, p1, p2);
    point.x > x_on_the_line
}

pub fn is_point_above_the_line(point: Vec3, p1: Vec3, p2: Vec3) -> bool {
    let z_on_the_line = calc_line_z(point.x, p1, p2);
    point.z < z_on_the_line
}

pub fn is_point_below_the_line(point: Vec3, p1: Vec3, p2: Vec3) -> bool {
    let z_on_the_line = calc_line_z(point.x, p1, p2);
    point.z > z_on_the_line
}


