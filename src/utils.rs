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

pub fn from_argb8(argb: u32) -> (u8, u8, u8, u8) {
    let a: u8 = (argb >> 24) as u8;
    let r: u8 = (argb >> 16) as u8;
    let g: u8 = (argb >> 8) as u8;
    let b: u8 = argb as u8;
    (a, r, g, b)
}

pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
}

pub fn clear_screen(buffer: &mut Vec<u32>, color: Vec4) {
    for pixel in buffer {
        *pixel = to_argb8(color.w as u8, color.x as u8, color.y as u8, color.z as u8);
    }
}

pub fn clear_buffer<T>(buffer: &mut [T], value: T)
where
    T: Copy,
{
    // will "consume" the iterator and return the n of iterations
    buffer.iter_mut().map(|x| *x = value).count();
}

pub fn edge_fn(v1: Vec2, v0: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    // instead of 3 divisions we can do 1/area *
    let a = 1.0 / area;

    // we can calculate 2 :) m0 + m1 + me = 1
    let m0 = edge_fn(v1, v2, point) * a;
    let m1 = edge_fn(v2, v0, point) * a;
    let m2 = edge_fn(v0, v1, point) * a;

    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(glam::vec3(m0, m1, m2))
    } else {
        None
    }
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}
