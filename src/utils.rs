use glam::{Vec2, Vec3, Vec4};

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn clear_screen(buffer: &mut Vec<u32>, color: Vec4) {
    for i in 0..buffer.len() {
        buffer[i] = to_argb8(color.w as u8, color.x as u8, color.y as u8, color.z as u8);
    }
}

pub fn edge_fn(p: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let m0 = edge_fn(point, v1, v2);
    let m1 = edge_fn(point, v2, v0);
    let m2 = edge_fn(point, v0, v1);
    // instead of 3 divisions we can do 1/area *
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        let a = 1.0 / area;
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}
