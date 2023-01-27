use glam::{Vec2, Vec3, Vec3Swizzles, Vec4};

use crate::utils::*;

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

    pub fn intersects(&self, point: Vec2) -> bool {
        point.x as f32 >= self.min.x
            && point.x as f32 <= self.max.x
            && point.y as f32 >= self.min.y
            && point.y as f32 <= self.max.y
    }
}

pub trait Shape {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>);
    fn get_area(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
}

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub bounding_box: AABB,
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Triangle {
            vertices,
            bounding_box: AABB::new(&vertices),
        }
    }
}

impl Shape for Triangle {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>) {
        let v0 = &self.vertices[0];
        let v1 = &self.vertices[1];
        let v2 = &self.vertices[2];

        for i in 0..buffer.len() {
            let coords = index_to_coords(i, WIDTH);

            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            if !(self.bounding_box.intersects(coords)) {
                continue;
            }

            let area = self.get_area();

            if let Some(bary) =
                barycentric_coordinates(coords, v0.position, v1.position, v2.position, area)
            {
                let depth = bary.x * v0.position.z + bary.y * v1.position.z + bary.z * v2.position.z;

                if depth <= depth_buffer[i] {
                    depth_buffer[i] = depth;

                    let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                    buffer[i] = to_argb8(
                        color.w as u8,
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                );
                }
            }
        }
    }

    fn get_area(&self) -> f32 {
        edge_fn(
            self.vertices[0].position.xy(),
            self.vertices[1].position,
            self.vertices[2].position,
        )
    }
}

pub struct Circle {
    pub radius: f32,
    pub center: Vec3,
    pub color: Vec4,
}

impl Shape for Circle {
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
