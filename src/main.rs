use glam::{Vec2, Vec4};
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;

use rusterizer::*;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_near: 4.0,
        frustum_far: 100.0,
        ..Default::default()
    };

    let vertices1: [Vertex; 4] = [
        Vertex {
            position: Vec4::new(-1.0, -1.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, 1.0, 1.0),
            color: Vec4::new(0.0, 0.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];

    //+z
    let transform0 = Transform::IDENTITY;
    //-z
    let transform1 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::PI,
        0.0,
        0.0,
    ));
    //+y
    let transform2 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //-y
    let transform3 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //+x
    let transform4 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        -std::f32::consts::FRAC_PI_2,
        0.0,
    ));
    //-x
    let transform5 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        std::f32::consts::FRAC_PI_2,
        0.0,
    ));

    let transforms: [Transform; 6] = [
        transform0, transform1, transform2, transform3, transform4, transform5,
    ];

    let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
    let vertices = vec![vertices1[0], vertices1[1], vertices1[2], vertices1[3]];

    let texture_path = String::from("resources/textures/phi.jpg");
    let texture: Arc<Texture> = Arc::new(Texture::load(std::path::Path::new(&texture_path)));

    let mut mesh0 = Mesh::from_vertices(&triangles, &vertices);
    let mut mesh1 = Mesh::from_vertices(&triangles, &vertices);
    let mut mesh2 = Mesh::from_vertices(&triangles, &vertices);
    let mut mesh3 = Mesh::from_vertices(&triangles, &vertices);
    let mut mesh4 = Mesh::from_vertices(&triangles, &vertices);
    let mut mesh5 = Mesh::from_vertices(&triangles, &vertices);
    mesh0.add_texture(texture.clone());
    mesh1.add_texture(texture.clone());
    mesh2.add_texture(texture.clone());
    mesh3.add_texture(texture.clone());
    mesh4.add_texture(texture.clone());
    mesh5.add_texture(texture.clone());

    let mut shapes: Vec<Box<dyn Object>> = vec![];
    shapes.push(Box::new(mesh0));
    shapes.push(Box::new(mesh1));
    shapes.push(Box::new(mesh2));
    shapes.push(Box::new(mesh3));
    shapes.push(Box::new(mesh4));
    shapes.push(Box::new(mesh5));

    let mut win_opts = WindowOptions::default();
    win_opts.resize = true;

    let mut window =
        Window::new("Test - ESC to exit", WIDTH, HEIGHT, win_opts).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut current_time = std::time::Instant::now();

    let mut rot = std::f32::consts::FRAC_PI_4;

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

        let parent_local =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, 0.0, 0.0))
                .local();

        let mut i = 0;
        // Draw shapes
        for shape in shapes.iter() {
            shape.draw(
                &mut buffer,
                &mut depth_buffer,
                &(camera.projection() * camera.view() * parent_local * transforms[i].local()),
                Vec2 {
                    x: WIDTH as f32,
                    y: HEIGHT as f32,
                },
            );
            i += 1;
        }
        rot += 0.05;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
