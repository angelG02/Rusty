use crate::transform::Transform;
use crate::{mesh::Mesh, Object};
use glam::{Mat4, Quat, Vec2, Vec3};
use std::path::Path;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub transform: Transform,
}

impl Model {
    pub fn new(file_path: &Path) -> Self {
        let (document, buffers, _images) = gltf::import(file_path).unwrap();

        let mut meshes: Vec<Mesh> = Vec::new();
        let mut transform: Transform = Transform::new(Vec3::ONE, Quat::IDENTITY, Vec3::ONE);

        for scene in document.scenes() {
            for node in scene.nodes() {
                println!(
                    "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                    node.index(),
                    node.children().count(),
                    node.camera(),
                    node.mesh().is_some(),
                    node.transform(),
                );

                transform.translation = Vec3::new(
                    node.transform().decomposed().0[0],
                    node.transform().decomposed().0[1],
                    node.transform().decomposed().0[2],
                );
                transform.rotation = Quat::from_array(node.transform().decomposed().1);
                transform.scale = Vec3::new(
                    node.transform().decomposed().2[0],
                    node.transform().decomposed().2[1],
                    node.transform().decomposed().2[2],
                );

                if let Some(mesh) = node.mesh() {
                    meshes.push(Mesh::new_from_gltf(&mesh, &buffers));
                }
            }
        }
        Model { meshes, transform }
    }

    pub fn draw(
        &self,
        buffer: &mut Vec<u32>,
        depth_buffer: &mut Vec<f32>,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
        for mesh in self.meshes.clone() {
            mesh.draw(
                buffer,
                depth_buffer,
                &self.transform.local(),
                &(*mvp * self.transform.local()),
                viewport_size,
            );
        }
    }
}
