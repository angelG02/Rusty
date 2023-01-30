use minifb::{Key, Window, WindowOptions};
use glam::{Vec2, Vec3, Vec4};
use std::sync::Arc;

use rusterizer::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_far: 100.0,
        ..Default::default()
    };

    let vertices1: [Vertex; 4] = [
        Vertex {
            position: Vec3::new(-1.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-1.0, 0.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.0, 0.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];

    let triangles = vec![glam::uvec3(0, 1, 2), glam::uvec3(0, 2, 3)];
    let vertices = vec![vertices1[0], vertices1[1], vertices1[2], vertices1[3]];

    let texture_path = String::from("resources/textures/logo1.png");
    let texture: Arc<Texture> = Arc::new(Texture::load(std::path::Path::new(&texture_path)));

    let mut mesh = Mesh::from_vertices(&triangles, &vertices);
    mesh.add_texture(texture);

    let mut shapes: Vec<Box<dyn Object>> = vec![];
    shapes.push(Box::new(mesh));

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
        // Clear screen
        let clear_color = Vec4::new(204.0, 255.0, 255.0, 0.0);
        clear_screen(&mut buffer, clear_color);
        clear_buffer(&mut depth_buffer, f32::INFINITY);

        // Calculate frame time (delta time)
        let new_time = std::time::Instant::now();
        let frame_time = (new_time - current_time).as_secs_f64();
        current_time = new_time;

        println!("Frame time: {frame_time}");

        // Update
        camera.update(&window, frame_time as f32);
        let mvp = camera.projection() * camera.view() * Transform::IDENTITY.local();        

        // Draw shapes
        for shape in shapes.iter() {
            shape.draw(
                &mut buffer,
                &mut depth_buffer,
                &mvp,
                Vec2 {
                    x: WIDTH as f32,
                    y: HEIGHT as f32,
                },
            );
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
