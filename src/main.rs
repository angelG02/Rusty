use glam::{Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};

pub mod utils;
pub use utils::*;

pub mod geometry;
pub use geometry::*;

pub mod texture;
pub use texture::Texture;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    // let vertices1: [Vertex; 4] = [
    //     Vertex {
    //         position: Vec3::new(0.0, 0.0, 1.0),
    //         color: Vec4::new(0.0, 0.0, 1.0, 0.0),
    //     },
    //     Vertex {
    //         position: Vec3::new(0.0, 300.0, 1.0),
    //         color: Vec4::new(0.0, 0.0, 1.0, 0.0),
    //     },
    //     Vertex {
    //         position: Vec3::new(300.0, 300.0, 1.0),
    //         color: Vec4::new(0.0, 0.0, 1.0, 0.0),
    //     },
    //     Vertex {
    //         position: Vec3::new(300.0, 0.0, 1.0),
    //         color: Vec4::new(0.0, 0.0, 1.0, 0.0),
    //     },
    // ];

    // let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vertices1: [Vertex; 3] = [
        Vertex {
            position: Vec3::new(0.0, 0.0, 1.0),
            color: Vec4::new(1.0, 0.0, 1.0, 0.0),
            uv: Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.0, 300.0, 1.0),
            color: Vec4::new(1.0, 0.0, 1.0, 0.0),
            uv: Vec3::new(0.5, 1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(300.0, 300.0, 1.0),
            color: Vec4::new(1.0, 0.0, 1.0, 0.0),
            uv: Vec3::new(1.0, 0.0, 1.0),
        },
    ];

    let texture_path = String::from("resources/textures/bojan.jpg");

    let triangle1: Triangle = Triangle::new_with_texture(vertices1, &texture_path);

    let mut shapes: Vec<Box<dyn Shape>> = vec![];
    shapes.push(Box::new(triangle1));

    let mut win_opts = WindowOptions::default();
    win_opts.resize = true;

    let mut window =
        Window::new("Test - ESC to exit", WIDTH, HEIGHT, win_opts).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut current_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calculate frame time (delta time)
        let new_time = std::time::Instant::now();
        let frame_time = (new_time - current_time).as_secs_f64();
        current_time = new_time;

        println!("Frame time: {frame_time}");

        // Clear screen
        let clear_color = Vec4::new(204.0, 255.0, 255.0, 0.0);
        clear_screen(&mut buffer, clear_color);

        // Draw shapes
        for shape in shapes.iter() {
            shape.draw(&mut buffer, &mut depth_buffer);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
