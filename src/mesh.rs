use crate::geometry::*;
use crate::texture::*;

use glam::{Mat4, UVec3, Vec2, Vec3, Vec4};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
    texture: Option<Arc<Texture>>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
            texture: None,
        }
    }

    pub fn new_with_texture(texture: Arc<Texture>) -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
            texture: Some(texture),
        }
    }

    pub fn new_from_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Mesh {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut indices = vec![];

        let mut result = Mesh::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| indices.push(i));
            }
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])))
            }

            let colors: Vec<Vec4> = positions.iter().map(|_| Vec4::ONE).collect();
            println!("Num vertices: {:?}", positions.len() * 3);
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());

            let triangles: Vec<UVec3> = indices
                .chunks_exact(3)
                .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
                .collect();
            result.add_section_from_buffers(&triangles, &positions, &normals, &colors, &tex_coords)
        }

        result
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

    pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_section_from_vertices(triangles, vertices);
        mesh
    }

    pub fn add_texture(&mut self, texture: Arc<Texture>) {
        self.texture = Some(texture);
    }

    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let triangles: Vec<UVec3> = triangles.iter().map(|tri| *tri + offset).collect();
        self.triangles.extend_from_slice(&triangles);
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_section_from_buffers(
        &mut self,
        triangles: &[UVec3],
        positions: &[Vec3],
        normals: &[Vec3],
        colors: &[Vec4],
        uvs: &[Vec2],
    ) {
        self.triangles.extend_from_slice(triangles);

        let has_uvs = !uvs.is_empty();
        let has_colors = !colors.is_empty();

        for i in 0..positions.len() {
            let vertex = Vertex::new(
                positions[i].extend(1.0),
                normals[i],
                if has_colors { colors[i] } else { Vec4::ONE },
                if has_uvs { uvs[i] } else { Vec2::ONE },
            );
            self.vertices.push(vertex);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Mesh {
    fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        model: &Mat4,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
        for triangle_indices in self.triangles.clone() {
            let triangle_vertices: [Vertex; 3] = self.get_vertices_from_triangle(triangle_indices);
            if self.texture.is_some() {
                let triangle = Triangle::new_with_texture(
                    triangle_vertices,
                    self.texture.as_ref().unwrap().clone(),
                );
                triangle.draw(buffer, depth_buffer, model, mvp, viewport_size);
            } else {
                let triangle = Triangle::new(triangle_vertices);
                triangle.draw(buffer, depth_buffer, model, mvp, viewport_size);
            }
        }
    }

    fn get_area(&self) -> f32 {
        let mut area: f32 = 0.0;
        for triangle_indices in self.triangles.clone() {
            let triangle_vertices: [Vertex; 3] = self.get_vertices_from_triangle(triangle_indices);
            let triangle = Triangle::new(triangle_vertices);
            area += triangle.get_area();
        }
        area
    }
}
