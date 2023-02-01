use crate::texture::*;
use crate::utils::*;
use glam::{Mat4, Vec2, Vec3, Vec4Swizzles, Vec4};
use std::sync::Arc;

use std::ops::{Add, Mul, Sub, MulAssign};

pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(vertices: &[Vec2; 3]) -> Self {
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        let xmax = f32::max(f32::max(v0.x, v1.x), v2.x);
        let ymax = f32::max(f32::max(v0.y, v1.y), v2.y);
        let xmin = f32::min(f32::min(v0.x, v1.x), v2.x);
        let ymin = f32::min(f32::min(v0.y, v1.y), v2.y);

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
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        mvp: &Mat4,
        viewport_size: Vec2,
    );
    fn get_area(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec4, color: Vec4, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
        }
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

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub texture: Option<Arc<Texture>>,
}

pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Triangle {
            vertices,
            texture: None,
        }
    }

    pub fn new_with_texture(vertices: [Vertex; 3], texture: Arc<Texture>) -> Self {
        Triangle {
            vertices,
            texture: Some(texture),
        }
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let p0 = *matrix * self.vertices[0].position.xyz().extend(1.0);
        let p1 = *matrix * self.vertices[1].position.xyz().extend(1.0);
        let p2 = *matrix * self.vertices[2].position.xyz().extend(1.0);

        let mut result = self.clone();
        result.vertices[0].position = p0;
        result.vertices[1].position = p1;
        result.vertices[2].position = p2;

        result
    }

    pub fn reorder(&self, order: VerticesOrder) -> Self {
        match order {
            VerticesOrder::ABC => self.clone(),
            VerticesOrder::ACB => Self::new([self.vertices[0], self.vertices[2], self.vertices[1]]),
            VerticesOrder::BAC => Self::new([self.vertices[1], self.vertices[0], self.vertices[2]]),
            VerticesOrder::BCA => Self::new([self.vertices[1], self.vertices[2], self.vertices[0]]),
            VerticesOrder::CAB => Self::new([self.vertices[2], self.vertices[0], self.vertices[1]]),
            VerticesOrder::CBA => Self::new([self.vertices[2], self.vertices[1], self.vertices[0]]),
        }
    }

    pub fn draw_clipped(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        viewport_size: Vec2,
    ) {
        let rec0 = 1.0 / self.vertices[0].position.w;
        let rec1 = 1.0 / self.vertices[1].position.w;
        let rec2 = 1.0 / self.vertices[2].position.w;

        // This would be the output of the vertex shader (clip space)
        // then we perform perspective division to transform in ndc
        // now x,y,z componend of ndc are between -1 and 1
        let ndc0 = self.vertices[0].position * rec0;
        let ndc1 = self.vertices[1].position * rec1;
        let ndc2 = self.vertices[2].position * rec2; 

        let v0 = self.vertices[0] * rec0;
        let v1 = self.vertices[1] * rec1;
        let v2 = self.vertices[2] * rec2;

        // screeen coordinates remapped to window
        let sc0 = glam::vec2(
            map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc1 = glam::vec2(
            map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc2 = glam::vec2(
            map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
        );

        let bounding_box: AABB = AABB::new(&[sc0, sc1, sc2]);

        let area = edge_fn(sc0, sc1, sc2);

        for i in 0..buffer.len() {
            let coords = index_to_coords(i, viewport_size.x as usize);

            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            if !(bounding_box.intersects(coords)) {
                continue;
            }

            if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                let depth = correction;
                let correction = 1.0 / correction;

                if depth < depth_buffer[i] {
                    depth_buffer[i] = depth;

                    let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                    let color = color * correction;

                    let mut color = to_argb8(
                        255,
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    );

                    if self.texture.is_some() {
                        let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                        let tex_coords = tex_coords * correction;

                        color = self
                            .texture
                            .as_ref()
                            .unwrap()
                            .argb_at_uv(tex_coords.x, tex_coords.y);
                    }
                    buffer[i] = color;
                }
            }
        }
    }
}

impl Object for Triangle {
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
        let clip_triangle = self.transform(mvp);

        match cull_triangle_view_frustum(&clip_triangle) {
            ClipResult::None => {
                return;
            }
            ClipResult::One(tri) => {
                tri.draw_clipped(buffer, depth_buffer, viewport_size);
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

pub enum ClipResult {
    None,
    One(Triangle),
}

//View Frustum Culling
pub fn cull_triangle_view_frustum(triangle: &Triangle) -> ClipResult {
    // cull tests against the 6 planes
    if triangle.vertices[0].position.x > triangle.vertices[0].position.w
        && triangle.vertices[1].position.x > triangle.vertices[1].position.w
        && triangle.vertices[2].position.x > triangle.vertices[2].position.w
    {
        return ClipResult::None;
    }
    if triangle.vertices[0].position.x < -triangle.vertices[0].position.w
        && triangle.vertices[1].position.x < -triangle.vertices[1].position.w
        && triangle.vertices[2].position.x < -triangle.vertices[2].position.w
    {
        return ClipResult::None;
    }
    if triangle.vertices[0].position.y > triangle.vertices[0].position.w
        && triangle.vertices[1].position.y > triangle.vertices[1].position.w
        && triangle.vertices[2].position.y > triangle.vertices[2].position.w
    {
        return ClipResult::None;
    }
    if triangle.vertices[0].position.y < -triangle.vertices[0].position.w
        && triangle.vertices[1].position.y < -triangle.vertices[1].position.w
        && triangle.vertices[2].position.y < -triangle.vertices[2].position.w
    {
        return ClipResult::None;
    }
    if triangle.vertices[0].position.z > triangle.vertices[0].position.w
        && triangle.vertices[1].position.z > triangle.vertices[1].position.w
        && triangle.vertices[2].position.z > triangle.vertices[2].position.w
    {
        return ClipResult::None;
    }
    if triangle.vertices[0].position.z < 0.0 && triangle.vertices[1].position.z < 0.0 && triangle.vertices[2].position.z < 0.0
    {
        return ClipResult::None;
    }

    ClipResult::One(triangle.clone())
}

pub struct Quad {
    vertices: [Vertex; 4],
    indices: [u32; 6],
    texture: Option<Arc<Texture>>,
}

impl Quad {
    pub fn new(vertices: [Vertex; 4], indices: [u32; 6]) -> Self {
        Quad {
            vertices,
            indices,
            texture: None,
        }
    }

    pub fn new_with_texture(
        vertices: [Vertex; 4],
        indices: [u32; 6],
        texture: Arc<Texture>,
    ) -> Self {
        Quad {
            vertices,
            indices,
            texture: Some(texture),
        }
    }
}

impl Object for Quad {
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
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

            triangle1.draw(buffer, depth_buffer, mvp, viewport_size);
            triangle2.draw(buffer, depth_buffer, mvp, viewport_size);
        } else {
            let triangle1 = Triangle::new_with_texture(
                triangle_vertices1,
                self.texture.as_ref().unwrap().clone(),
            );
            let triangle2 = Triangle::new_with_texture(
                triangle_vertices2,
                self.texture.as_ref().unwrap().clone(),
            );

            triangle1.draw(buffer, depth_buffer, mvp, viewport_size);
            triangle2.draw(buffer, depth_buffer, mvp, viewport_size);
        }
    }

    fn get_area(&self) -> f32 {
        f32::INFINITY
    }
}

pub struct Circle {
    pub radius: f32,
    pub center: Vec3,
    pub color: Vec4,
}

impl Object for Circle {
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        _mvp: &Mat4,
        _viewport_size: Vec2,
    ) {
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
