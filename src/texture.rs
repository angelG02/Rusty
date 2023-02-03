use crate::utils::*;
use glam::Vec4;
use image::{self, GenericImageView};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    //pub name: String,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    //pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = image::open(path).expect("File not found");

        let image_pixels = decoded_image.pixels();

        let mut data: Vec<u32> = vec![];

        for (index, (_width, _height, pixel)) in image_pixels.enumerate() {
            let a = pixel.0[3];
            let r = pixel.0[0];
            let g = pixel.0[1];
            let b = pixel.0[2];
            data.insert(index, to_argb8(a, r, g, b));
        }

        Texture {
            width: decoded_image.width() as usize,
            height: decoded_image.height() as usize,
            data,
        }
    }

    pub fn uv_to_index(&self, u: f32, v: f32) -> usize {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        coords_to_index(
            (u as usize) % self.width,
            (v as usize) % self.height,
            self.width,
        )
    }

    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let id = self.uv_to_index(u, v);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }

    pub fn argb_at_uvf(&self, u: f32, v: f32) -> Vec4 {
        let id = self.uv_to_index(u, v);
        if id < self.data.len() {
            let color = from_argb8(self.data[id]);
            Vec4::new(
                (color.0 as f32) / 255.0,
                (color.1 as f32) / 255.0,
                (color.2 as f32) / 255.0,
                (color.3 as f32) / 255.0,
            )
        } else {
            Vec4::new(1.0, 1.0, 0.0, 1.0)
        }
    }
}
