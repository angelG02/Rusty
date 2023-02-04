use glam::{Vec2, Vec4};
use minifb::{Key, MouseButton, Window, WindowOptions};
use std::cell::UnsafeCell;
use std::path::Path;

use rusterizer::*;

const ROW_CHUNK_SIZE: usize = (WIDTH * HEIGHT) / 16;
const GRID_SIZE: usize = (WIDTH * HEIGHT) / 16;

fn main() {
    let mut buffer: UnsafeCell<Vec<u32>> = vec![0; WIDTH * HEIGHT].into();
    let mut depth_buffer: UnsafeCell<Vec<f32>> = vec![f32::INFINITY; WIDTH * HEIGHT].into();

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_near: 1.0,
        frustum_far: 100.0,
        ..Default::default()
    };

    let screen_buffer_chuncks: UnsafeCell<Vec<&mut [u32]>> = vec![].into();
    let depth_buffer_chuncks: UnsafeCell<Vec<&mut [f32]>> = vec![].into();

    unsafe {
        for col in (*buffer.get()).chunks_mut(WIDTH * ROW_CHUNK_SIZE) {
            for row in col.chunks_mut(ROW_CHUNK_SIZE) {
                for chunk in row.chunks_mut(GRID_SIZE) {
                    (*screen_buffer_chuncks.get()).push(chunk);
                }
            }
        }

        for col in (*depth_buffer.get()).chunks_mut(WIDTH * ROW_CHUNK_SIZE) {
            for row in col.chunks_mut(ROW_CHUNK_SIZE) {
                for chunk in row.chunks_mut(GRID_SIZE) {
                    (*depth_buffer_chuncks.get()).push(chunk);
                }
            }
        }
    }

    let thread_pool = ThreadPool::new(32);

    let texture = std::sync::Arc::new(Texture::load(std::path::Path::new(
        "resources/models/SciFiHelmet/SciFiHelmet_BaseColor.png",
    )));
    let mut helmet: Model = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));
    helmet.meshes[0].add_texture(texture);

    // let helmet1: Model = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));
    // let helmet2: Model = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));
    // let helmet3: Model = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));
    // let helmet4: Model = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));

    // let mut helmets: Vec<Model> = vec![];
    // helmets.push(helmet.clone());
    // helmets.push(helmet1.clone());
    // helmets.push(helmet2.clone());
    // helmets.push(helmet3.clone());
    // helmets.push(helmet4.clone());

    let win_opts = WindowOptions {
        resize: false,
        ..Default::default()
    };

    let mut window = Window::new("Rusty", WIDTH, HEIGHT, win_opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut current_time = std::time::Instant::now();

    let mut rot = std::f32::consts::FRAC_PI_4;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calculate frame time (delta time)
        let new_time = std::time::Instant::now();
        let frame_time = (new_time - current_time).as_millis();
        let delta_time = frame_time as f32 / 1000.0;
        current_time = new_time;

        //println!("Frame time: {frame_time}ms");

        let raster_time = std::time::Instant::now();

        // Clear screen and depth buffer
        let clear_color = Vec4::new(0.0, 0.0, 0.0, 0.0);

        unsafe {
            // Multi-threaded clearing of the screen and depth buffers xddd
            let mut color_offset = 0.0;
            let chunks = &mut *screen_buffer_chuncks.get();
            for chunk in chunks {
                thread_pool.execute(move || {
                    clear_screen(
                        chunk,
                        Vec4::new(
                            clear_color.x + color_offset,
                            clear_color.y,
                            clear_color.z + color_offset,
                            clear_color.w,
                        ),
                    );
                });
                color_offset += 15.0;
            }

            let chunks = &mut *depth_buffer_chuncks.get();
            for chunk in chunks {
                thread_pool.execute(move || {
                    clear_buffer(&mut *chunk, f32::INFINITY);
                });
            }
        }
        // Update
        camera.update(&window, delta_time);

        helmet.transform =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, rot, 0.0));
        let mvp = camera.projection() * camera.view() * helmet.transform.local();

        // Draw shapes
        helmet.draw(
            buffer.get_mut(),
            depth_buffer.get_mut(),
            &mvp,
            Vec2 {
                x: WIDTH as f32,
                y: HEIGHT as f32,
            },
        );

        // Render-only time
        let raster_time = raster_time.elapsed().as_millis();
        println!("Render time: {raster_time}ms");

        if window.get_mouse_down(MouseButton::Left) {
            rot += 1.0 * delta_time;
        } else if window.get_mouse_down(MouseButton::Right) {
            rot -= 1.0 * delta_time;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        unsafe {
            window
                .update_with_buffer(&*buffer.get(), WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
