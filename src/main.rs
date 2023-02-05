use glam::{Vec2, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use std::cell::UnsafeCell;
use std::path::Path;
use std::sync::{Arc, Mutex};

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

    let thread_pool = ThreadPool::new(16);

    let texture = Arc::new(Texture::load(std::path::Path::new(
        "resources/models/SciFiHelmet/SciFiHelmet_BaseColor.png",
    )));

    let objects: Arc<Mutex<Vec<Model>>> = Arc::new(Mutex::new(vec![]));

    let mut model_trans = Vec3::new(0.0, 0.0, 0.0);
    for _i in 0..10 {
        let objects = Arc::clone(&objects);
        let texture = Arc::clone(&texture);
        thread_pool.execute(move || {
            let mut helm = Model::new(Path::new("resources/models/SciFiHelmet/SciFiHelmet.gltf"));
            helm.transform = Transform::from_translation(model_trans);
            helm.meshes[0].add_texture(texture);

            objects.lock().unwrap().push(helm);
        });
        model_trans.x += 1.0;
    }

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

    //let mut rot = std::f32::consts::FRAC_PI_4;

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
                    clear_buffer(chunk, f32::INFINITY);
                });
            }
        }
        // Update
        camera.update(&window, delta_time);

        if !objects.lock().unwrap().is_empty() {
            let objects = Arc::clone(&objects);
            for object in &*objects.lock().unwrap() {
                let mvp = camera.projection() * camera.view() * object.transform.local();

                // Draw objects
                object.draw(
                    buffer.get_mut(),
                    depth_buffer.get_mut(),
                    &mvp,
                    Vec2 {
                        x: WIDTH as f32,
                        y: HEIGHT as f32,
                    },
                );
            }
        }

        // Render-only time
        let raster_time = raster_time.elapsed().as_millis();
        println!("Render time: {raster_time}ms");

        // if window.get_mouse_down(MouseButton::Left) {
        //     rot += 1.0 * delta_time;
        // } else if window.get_mouse_down(MouseButton::Right) {
        //     rot -= 1.0 * delta_time;
        // }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        unsafe {
            window
                .update_with_buffer(&*buffer.get(), WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
