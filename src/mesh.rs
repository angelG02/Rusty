use crate::geometry::*;
use crate::texture::*;

use glam::UVec3;
use std::sync::Arc;

pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
    texture: Option<Arc<Texture>>
}

impl Mesh {
    pub fn new() -> Self {
        Self { triangles: Vec::new(), vertices: Vec::new(), texture: None }
    }

    pub fn new_with_texture(texture: Arc<Texture>) -> Self {
        Self { triangles: Vec::new(), vertices: Vec::new(), texture: Some(texture) }
    }

    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [Vertex; 3] {
        [
            self.vertices[triangle.x as usize],
            self.vertices[triangle.y as usize],
            self.vertices[triangle.z as usize],
        ]
    }

    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        self.triangles.extend_from_slice(triangles);
        self.vertices.extend_from_slice(vertices);
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Mesh {
    fn draw(&self, buffer: &mut Vec<u32>, depth_buffer: &mut Vec<f32>) {
        for triangle_indices in self.triangles.clone() {
            let triangle_vertices: [Vertex; 3] = self.get_vertices_from_triangle(triangle_indices);
            if self.texture.is_some() {
                let triangle = Triangle::new_with_texture(triangle_vertices, self.texture.as_ref().unwrap().clone());
                triangle.draw(buffer, depth_buffer)
            } else {
                let triangle = Triangle::new(triangle_vertices);
                triangle.draw(buffer, depth_buffer)
            }            
        }
    }

    fn get_area(&self) -> f32 {
        f32::INFINITY
    }
}