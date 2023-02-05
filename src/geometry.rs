use crate::texture::*;
use crate::utils::*;
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use std::sync::Arc;

use std::ops::{Add, Mul, MulAssign, Sub};

pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(vertices: &[Vec2; 3]) -> Self {
        let left = vertices[0].x.min(vertices[1].x).min(vertices[2].x);
        let right = vertices[0].x.max(vertices[1].x).max(vertices[2].x);
        let bottom = vertices[0].y.min(vertices[1].y).min(vertices[2].y);
        let top = vertices[0].y.max(vertices[1].y).max(vertices[2].y);

        AABB {
            min: Vec2::new(left, bottom),
            max: Vec2::new(right, top),
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
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

// this takes care of raster clipping
pub fn triangle_screen_bounding_box(positions: &[Vec2; 3], viewport_size: Vec2) -> Option<AABB> {
    let bb = AABB::new(positions);

    if bb.min.x > viewport_size.x || bb.max.x < 0.0 || bb.min.y > viewport_size.y || bb.max.y < 0.0
    {
        None
    } else {
        let left = bb.min.x.max(0.0);
        let right = bb.max.x.min(viewport_size.x - 1.0);
        let bottom = bb.min.y.max(0.0);
        let top = bb.max.y.min(viewport_size.y - 1.0);

        Some(AABB {
            min: Vec2 { x: left, y: bottom },
            max: Vec2 { x: right, y: top },
        })
    }
}

pub trait Object {
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        model: &Mat4,
        mvp: &Mat4,
        viewport_size: Vec2,
    );
    fn get_area(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub normal: Vec3,
    pub color: Vec4,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec4, normal: Vec3, color: Vec4, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            color,
            uv,
        }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let position = self.position + rhs.position;
        let normal = self.normal + rhs.normal;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let position = self.position - rhs.position;
        let normal = self.normal - rhs.normal;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Mul for Vertex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let position = self.position * rhs.position;
        let normal = self.normal * rhs.normal;
        let color = self.color * rhs.color;
        let uv = self.uv * rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, normal, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.normal *= rhs;
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
            VerticesOrder::ACB => {
                if self.texture.is_some() {
                    Self::new_with_texture(
                        [self.vertices[0], self.vertices[2], self.vertices[1]],
                        self.texture.clone().unwrap(),
                    )
                } else {
                    Self::new([self.vertices[0], self.vertices[2], self.vertices[1]])
                }
            }

            VerticesOrder::BAC => {
                if self.texture.is_some() {
                    Self::new_with_texture(
                        [self.vertices[1], self.vertices[0], self.vertices[2]],
                        self.texture.clone().unwrap(),
                    )
                } else {
                    Self::new([self.vertices[1], self.vertices[0], self.vertices[2]])
                }
            }
            VerticesOrder::BCA => {
                if self.texture.is_some() {
                    Self::new_with_texture(
                        [self.vertices[1], self.vertices[2], self.vertices[0]],
                        self.texture.clone().unwrap(),
                    )
                } else {
                    Self::new([self.vertices[1], self.vertices[2], self.vertices[0]])
                }
            }
            VerticesOrder::CAB => {
                if self.texture.is_some() {
                    Self::new_with_texture(
                        [self.vertices[2], self.vertices[0], self.vertices[1]],
                        self.texture.clone().unwrap(),
                    )
                } else {
                    Self::new([self.vertices[2], self.vertices[0], self.vertices[1]])
                }
            }
            VerticesOrder::CBA => {
                if self.texture.is_some() {
                    Self::new_with_texture(
                        [self.vertices[2], self.vertices[1], self.vertices[0]],
                        self.texture.clone().unwrap(),
                    )
                } else {
                    Self::new([self.vertices[2], self.vertices[1], self.vertices[0]])
                }
            }
        }
    }

    pub fn draw_clipped(&self, buffer: &mut [u32], depth_buffer: &mut [f32], viewport_size: Vec2) {
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

        if let Some(bounding_box) = triangle_screen_bounding_box(&[sc0, sc1, sc2], viewport_size) {
            let area = edge_fn(sc0, sc1, sc2);

            for x in (bounding_box.min.x as usize)..=bounding_box.max.x as usize {
                for y in (bounding_box.min.y as usize)..=bounding_box.max.y as usize {
                    let coords = glam::vec2(x as f32, y as f32) + 0.5;
                    let pixel_id = coords_to_index(x, y, viewport_size.x as usize);

                    if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                        let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                        let depth = bary.x * ndc0.z + bary.y * ndc1.z + bary.z * ndc2.z;
                        let correction = 1.0 / correction;

                        if depth < depth_buffer[pixel_id] {
                            depth_buffer[pixel_id] = depth;

                            let normal =
                                bary.x * v0.normal + bary.y * v1.normal + bary.z * v2.normal;
                            let normal = normal * correction;
                            let n_dot_1 = normal.dot(Vec3::ONE.normalize());

                            let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                            let mut color = color * correction;

                            if let Some(tex) = &self.texture {
                                let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                                let tex_coords = tex_coords * correction;
                                color = tex
                                    .argb_at_uvf(tex_coords.x, tex_coords.y)
                                    .yzw()
                                    .extend(1.0);
                            }

                            let ambient = glam::vec4(0.2, 0.2, 0.2, 1.0);

                            color = color * n_dot_1 + ambient;

                            let out_color = to_argb8(
                                255,
                                (color.x * 255.0) as u8,
                                (color.y * 255.0) as u8,
                                (color.z * 255.0) as u8,
                            );

                            buffer[pixel_id] = out_color;
                        }
                    }
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
        model: &Mat4,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
        let cof_mat = cofactor(model);
        let mut clip_triangle = self.transform(mvp);

        clip_triangle.vertices[0].normal =
            (cof_mat * clip_triangle.vertices[0].normal.extend(0.0)).xyz();
        clip_triangle.vertices[1].normal =
            (cof_mat * clip_triangle.vertices[1].normal.extend(0.0)).xyz();
        clip_triangle.vertices[2].normal =
            (cof_mat * clip_triangle.vertices[2].normal.extend(0.0)).xyz();

        match clip_cull_triangle(&clip_triangle) {
            ClipResult::None => {}
            ClipResult::One(tri) => {
                tri.draw_clipped(buffer, depth_buffer, viewport_size);
            }
            ClipResult::Two(tri) => {
                tri.0.draw_clipped(buffer, depth_buffer, viewport_size);
                tri.1.draw_clipped(buffer, depth_buffer, viewport_size);
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
    Two((Triangle, Triangle)),
}

//View Frustum Culling
pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
    // cull tests against the 6 planes
    if triangle.vertices[0].position.x > triangle.vertices[0].position.w
        && triangle.vertices[1].position.x > triangle.vertices[1].position.w
        && triangle.vertices[2].position.x > triangle.vertices[2].position.w
    {
        return true;
    }
    if triangle.vertices[0].position.x < -triangle.vertices[0].position.w
        && triangle.vertices[1].position.x < -triangle.vertices[1].position.w
        && triangle.vertices[2].position.x < -triangle.vertices[2].position.w
    {
        return true;
    }
    if triangle.vertices[0].position.y > triangle.vertices[0].position.w
        && triangle.vertices[1].position.y > triangle.vertices[1].position.w
        && triangle.vertices[2].position.y > triangle.vertices[2].position.w
    {
        return true;
    }
    if triangle.vertices[0].position.y < -triangle.vertices[0].position.w
        && triangle.vertices[1].position.y < -triangle.vertices[1].position.w
        && triangle.vertices[2].position.y < -triangle.vertices[2].position.w
    {
        return true;
    }
    if triangle.vertices[0].position.z > triangle.vertices[0].position.w
        && triangle.vertices[1].position.z > triangle.vertices[1].position.w
        && triangle.vertices[2].position.z > triangle.vertices[2].position.w
    {
        return true;
    }
    if triangle.vertices[0].position.z < 0.0
        && triangle.vertices[1].position.z < 0.0
        && triangle.vertices[2].position.z < 0.0
    {
        return true;
    }

    false
}

pub fn clip_triangle_two(triangle: &Triangle) -> (Triangle, Triangle) {
    // calculate alpha values for getting adjusted vertices
    let alpha_a = (-triangle.vertices[0].position.z)
        / (triangle.vertices[1].position.z - triangle.vertices[0].position.z);
    let alpha_b = (-triangle.vertices[0].position.z)
        / (triangle.vertices[2].position.z - triangle.vertices[0].position.z);

    // interpolate to get v0a and v0b
    let v0_a = lerp(triangle.vertices[0], triangle.vertices[1], alpha_a);
    let v0_b = lerp(triangle.vertices[0], triangle.vertices[2], alpha_b);

    // draw triangles
    let mut result_a = triangle.clone();
    let mut result_b = triangle.clone();

    result_a.vertices[0] = v0_a;

    result_b.vertices[0] = v0_a;
    result_b.vertices[1] = v0_b;

    (result_a, result_b)
}

pub fn clip_triangle_one(triangle: &Triangle) -> Triangle {
    let alpha_a = (-triangle.vertices[0].position.z)
        / (triangle.vertices[2].position.z - triangle.vertices[0].position.z);
    let alpha_b = (-triangle.vertices[1].position.z)
        / (triangle.vertices[2].position.z - triangle.vertices[1].position.z);

    // interpolate to get v0a and v0b
    let mut v0 = lerp(triangle.vertices[0], triangle.vertices[2], alpha_a);
    let mut v1 = lerp(triangle.vertices[1], triangle.vertices[2], alpha_b);

    let mut v2 = triangle.vertices[2];

    let red = Vec3::new(1.0, 0.0, 0.0);

    v0.color = Vec4::new(red.x, red.y, red.z, 1.0);
    v1.color = Vec4::new(red.x, red.y, red.z, 1.0);
    v2.color = Vec4::new(red.x, red.y, red.z, 1.0);

    //println!("out tri: {:?}, {:?}, {:?},", v0, v1, v2);
    // draw triangles
    Triangle {
        vertices: [v0, v1, v2],
        texture: triangle.texture.clone(),
    }
}

pub fn cull_triangle_backface(triangle: &Triangle) -> bool {
    let normal = (triangle.vertices[1].position.xyz() - triangle.vertices[0].position.xyz())
        .cross(triangle.vertices[2].position.xyz() - triangle.vertices[0].position.xyz());
    // also we don't care about normalizing
    // if negative facing the camera
    normal.z <= 0.0
}

pub fn clip_cull_triangle(triangle: &Triangle) -> ClipResult {
    if cull_triangle_backface(triangle) {
        return ClipResult::None;
    }
    if cull_triangle_view_frustum(triangle) {
        ClipResult::None
    } else {
        // clipping routines
        if triangle.vertices[0].position.z < 0.0 {
            if triangle.vertices[1].position.z < 0.0 {
                ClipResult::One(clip_triangle_one(triangle))
            } else if triangle.vertices[2].position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::ACB)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::ACB)))
            }
        } else if triangle.vertices[1].position.z < 0.0 {
            if triangle.vertices[2].position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::BCA)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::BAC)))
            }
        } else if triangle.vertices[2].position.z < 0.0 {
            ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::CBA)))
        } else {
            // no near clipping necessary
            // return original
            ClipResult::One(triangle.clone())
        }
    }
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
        model: &Mat4,
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

            triangle1.draw(buffer, depth_buffer, model, mvp, viewport_size);
            triangle2.draw(buffer, depth_buffer, model, mvp, viewport_size);
        } else {
            let triangle1 = Triangle::new_with_texture(
                triangle_vertices1,
                self.texture.as_ref().unwrap().clone(),
            );
            let triangle2 = Triangle::new_with_texture(
                triangle_vertices2,
                self.texture.as_ref().unwrap().clone(),
            );

            triangle1.draw(buffer, depth_buffer, model, mvp, viewport_size);
            triangle2.draw(buffer, depth_buffer, model, mvp, viewport_size);
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
        _model: &Mat4,
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

            if d <= self.radius.into() && self.center.z < depth_buffer[i] {
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

    fn get_area(&self) -> f32 {
        std::f32::consts::PI * f32::powf(self.radius, 2.0)
    }
}
