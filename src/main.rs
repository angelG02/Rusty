use glam::{Vec2, Vec4};
use minifb::{Key, Window, WindowOptions};

use rusterizer::*;

const SPEED: f32 = 2.0;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_near: 1.0,
        frustum_far: 100.0,
        ..Default::default()
    };

    let win_opts = WindowOptions {
        resize: true,
        ..Default::default()
    };

    let mut window = Window::new("Rusty", WIDTH, HEIGHT, win_opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut current_time = std::time::Instant::now();

    let mut constant = Vec2 {
        x: -1.2804998,
        y: 0.05300001,
    };

    let mut scale_in = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear screen
        let clear_color = Vec4::new(204.0, 255.0, 255.0, 0.0);
        clear_screen(&mut buffer, clear_color);
        clear_buffer(&mut depth_buffer, f32::INFINITY);

        // Calculate frame time (delta time)
        let new_time = std::time::Instant::now();
        let frame_time = (new_time - current_time).as_millis();
        let delta_time = frame_time as f32 / 1000.0;
        current_time = new_time;

        //println!("Frame time: {frame_time}ms");

        // Update
        //camera.update(&window, delta_time);

        // Draw fractal
        fractal::draw(
            &mut buffer,
            Vec2 {
                x: WIDTH as f32,
                y: HEIGHT as f32,
            },
            constant,
            scale_in,
        );

        if window.is_key_down(Key::Z) {
            scale_in += delta_time * 50.0;
        }

        if window.is_key_down(Key::X) {
            scale_in -= delta_time * 50.0;
        }

        if window.is_key_down(Key::W) {
            constant.x += delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::S) {
            constant.x -= delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::A) {
            constant.y += delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::D) {
            constant.y -= delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::Up) {
            constant += delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::Down) {
            constant -= delta_time / SPEED;
            println!("{:?}", constant)
        }

        if window.is_key_down(Key::R) {
            constant = Vec2 { x: 0.0, y: 0.0 };
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
