use crate::texture::*;
use crate::utils::*;
use glam::{Vec2, Vec3, Vec3Swizzles, Vec4};

use std::ops::{Add, Mul, Sub};

pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(vertices: &[Vertex; 3]) -> Self {
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        let xmax = f32::max(f32::max(v0.position.x, v1.position.x), v2.position.x);
        let ymax = f32::max(f32::max(v0.position.y, v1.position.y), v2.position.y);
        let xmin = f32::min(f32::min(v0.position.x, v1.position.x), v2.position.x);
        let ymin = f32::min(f32::min(v0.position.y, v1.position.y), v2.position.y);

        AABB {
            min: Vec2::new(xmin, ymin),
            max: Vec2::new(xmax, ymax),
        }
    }

    pub fn new_box(vertices: &[Vertex; 4]) -> Self {
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];
        let v3 = vertices[3];

        let xmax = f32::max(
            f32::max(f32::max(v0.position.x, v1.position.x), v2.position.x),
            v3.position.x,
        );
        let ymax = f32::max(
            f32::max(f32::max(v0.position.y, v1.position.y), v2.position.y),
            v3.position.y,
        );
        let xmin = f32::max(
            f32::min(f32::min(v0.position.x, v1.position.x), v2.position.x),
            v3.position.x,
        );
        let ymin = f32::max(
            f32::min(f32::min(v0.position.y, v1.position.y), v2.position.y),
            v3.position.y,
        );

        AABB {
            min: Vec2::new(xmin, ymin),
            max: Vec2::new(xmax, ymax),
        }
    }

    pub fn intersects(&self, point: Vec2) -> bool {
        point.x as f32 >= self.min.x
            && point.x as f32 <= self.max.x
            && point.y as f32 >= self.min.y
            && point.y as f32 <= self.max.y
    }
}

pub trait Object {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>);
    fn get_area(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec4, uv: Vec2) -> Self {
        Self { position, color, uv }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Mul for Vertex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let position = self.position * rhs.position;
        let color = self.color * rhs.color;
        let uv = self.uv * rhs.uv;
        Self::new(position, color, uv)
    }
}

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub bounding_box: AABB,
    pub texture: Option<Texture>,
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Triangle {
            vertices,
            bounding_box: AABB::new(&vertices),
            texture: None,
        }
    }

    pub fn new_with_texture(vertices: [Vertex; 3], texture: Texture) -> Self {
        Triangle {
            vertices,
            bounding_box: AABB::new(&vertices),
            texture: Some(texture),
        }
    }
}

impl Object for Triangle {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>) {
        let v0 = &self.vertices[0];
        let v1 = &self.vertices[1];
        let v2 = &self.vertices[2];

        let area = self.get_area();

        for i in 0..buffer.len() {
            let coords = index_to_coords(i, WIDTH);

            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            if !(self.bounding_box.intersects(coords)) {
                continue;
            }

            if let Some(bary) =
                barycentric_coordinates(coords, v0.position.xy(), v1.position.xy(), v2.position.xy(), area)
            {
                let depth =
                    bary.x * v0.position.z + bary.y * v1.position.z + bary.z * v2.position.z;

                if depth <= depth_buffer[i] {
                    depth_buffer[i] = depth;

                    if self.texture.is_none() {
                        let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                        buffer[i] = to_argb8(
                            color.w as u8,
                            (color.x * 255.0) as u8,
                            (color.y * 255.0) as u8,
                            (color.z * 255.0) as u8,
                        );
                    } else {
                        let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                        let color = self
                            .texture
                            .as_ref()
                            .unwrap()
                            .argb_at_uv(tex_coords.x, tex_coords.y);

                        buffer[i] = color;
                    }
                }
            }
        }
    }

    fn get_area(&self) -> f32 {
        edge_fn(
            self.vertices[0].position.xy(),
            self.vertices[1].position.xy(),
            self.vertices[2].position.xy(),
        )
    }
}

pub struct Quad {
    vertices: [Vertex; 4],
    indices: [u32; 6],
    bounding_box: AABB,
    texture: Option<Texture>,
}

impl Quad {
    pub fn new(vertices: [Vertex; 4], indices: [u32; 6]) -> Self {
        Quad {
            vertices,
            indices,
            bounding_box: AABB::new_box(&vertices),
            texture: None,
        }
    }

    pub fn new_with_texture(vertices: [Vertex; 4], indices: [u32; 6], texture: Texture) -> Self {
        Quad {
            vertices,
            indices,
            bounding_box: AABB::new_box(&vertices),
            texture: Some(texture),
        }
    }
}

impl Object for Quad {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>) {
        let triangle_vertices1: [Vertex; 3] = [
            self.vertices[self.indices[0] as usize],
            self.vertices[self.indices[1] as usize],
            self.vertices[self.indices[2] as usize],
        ];
        let triangle_vertices2: [Vertex; 3] = [
            self.vertices[self.indices[3] as usize],
            self.vertices[self.indices[4] as usize],
            self.vertices[self.indices[5] as usize],
        ];

        if self.texture.is_none() {
            let triangle1 = Triangle::new(triangle_vertices1);
            let triangle2 = Triangle::new(triangle_vertices2);

            triangle1.draw(buffer, depth_buffer);
            triangle2.draw(buffer, depth_buffer);
        } else {
            let triangle1 = Triangle::new_with_texture(
                triangle_vertices1,
                self.texture.as_ref().unwrap().clone(),
            );
            let triangle2 = Triangle::new_with_texture(
                triangle_vertices2,
                self.texture.as_ref().unwrap().clone(),
            );

            triangle1.draw(buffer, depth_buffer);
            triangle2.draw(buffer, depth_buffer);
        }
    }

    fn get_area(&self) -> f32 {
        (self.bounding_box.max.x - self.bounding_box.min.x)
            * (self.bounding_box.max.y - self.bounding_box.min.y)
    }
}

pub struct Circle {
    pub radius: f32,
    pub center: Vec3,
    pub color: Vec4,
}

impl Object for Circle {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>) {
        for i in 0..buffer.len() {
            let x = i as f32 % WIDTH as f32;
            let y = i as f32 / WIDTH as f32;
            let point = Vec2::new(x, y);

            let d = f64::sqrt(
                f64::powf((point.x - self.center.x) as f64, 2.0)
                    + f64::powf((point.y - self.center.y) as f64, 2.0),
            );

            if d <= self.radius.into() {
                if self.center.z < depth_buffer[i] {
                    depth_buffer[i] = self.center.z;

                    buffer[i] = to_argb8(
                        self.color.w as u8,
                        self.color.x as u8,
                        self.color.y as u8,
                        self.color.z as u8,
                    );
                }
            }
        }
    }

    fn get_area(&self) -> f32 {
        std::f32::consts::PI * f32::powf(self.radius, 2.0)
    }
}
