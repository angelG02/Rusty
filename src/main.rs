use glam::{Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};

pub mod utils;
pub use utils::*;

pub mod geometry;
pub use geometry::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let vertices1: [Vertex; 3] = [
        Vertex {
            position: Vec3::new((WIDTH / 2) as f32, (HEIGHT / 4) as f32, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new((WIDTH / 4) as f32, (HEIGHT - (HEIGHT / 4)) as f32, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(
                (WIDTH - (WIDTH / 4)) as f32,
                (HEIGHT - (HEIGHT / 4)) as f32,
                1.0,
            ),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
    ];

    let vertices2: [Vertex; 3] = [
        Vertex {
            position: Vec3::new((WIDTH / 2) as f32 + 100.0, (HEIGHT / 4) as f32, 1.0),
            color: Vec4::new(1.0, 0.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new((WIDTH / 4) as f32 + 100.0, (HEIGHT - (HEIGHT / 4)) as f32, 1.0),
            color: Vec4::new(0.0, 1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(
                (WIDTH - (WIDTH / 4)) as f32 + 100.0,
                (HEIGHT - (HEIGHT / 4)) as f32,
                1.0,
            ),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
    ];

    let triangle1: Triangle = Triangle::new(vertices1);
    let triangle2: Triangle = Triangle::new(vertices2);

    let mut shapes: Vec<Box<dyn Shape>> = vec![];
    shapes.push(Box::new(triangle1));
    shapes.push(Box::new(triangle2));

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
