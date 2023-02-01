use crate::transform::Transform;

use glam::Mat4;
use minifb::{Key, Window};

pub struct Camera {
    pub frustum_near: f32,
    pub frustum_far: f32,
    pub fov: f32, // in radians
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            frustum_near: 0.1,
            frustum_far: 100.0,
            fov: std::f32::consts::PI / 4.0,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY,
            speed: 10.0,
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov,
            self.aspect_ratio,
            self.frustum_near,
            self.frustum_far,
        )
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }

    pub fn update(&mut self, window: &Window, dt: f32) {
        let mut axis = glam::vec2(0.0, 0.0);
        // we will make registering later
    
        if window.is_key_down(Key::A) {
            axis.x -= 1.0;
        }
        if window.is_key_down(Key::D) {
            axis.x += 1.0;
        }
        if window.is_key_down(Key::W) {
            axis.y -= 1.0;
        }
        if window.is_key_down(Key::S) {
            axis.y += 1.0;
        }
        self.transform.translation += self.transform.right() * self.speed * axis.x * dt
            + self.transform.forward() * self.speed * axis.y * dt;

        if window.is_key_down(Key::E) {
            self.transform.translation.y += self.speed * dt;
        }

        if window.is_key_down(Key::Q) {
            self.transform.translation.y -= self.speed * dt;
        }
    }
}
