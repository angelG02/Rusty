use crate::utils::*;
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
        let mut index = 0;

        for (_width, _height, pixel) in image_pixels {
            let a = pixel.0[3];
            let r = pixel.0[0];
            let g = pixel.0[1];
            let b = pixel.0[2];
            data.insert(index, to_argb8(a, r, g, b));
            index += 1;
        }

        Texture { width: decoded_image.width() as usize, height: decoded_image.height() as usize, data}
    }

    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        let id = coords_to_index(u as usize, v as usize, self.width);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }
}