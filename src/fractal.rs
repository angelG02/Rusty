// Escape time fractals Zn = (Zn-1)^2 + C

use super::utils::to_argb8;
use glam::Vec2;

const MAX_ITERATIONS: u32 = 500;

// Compute Zn^2 + C
pub fn compute_next(current: Vec2, constant: Vec2) -> Vec2 {
    // Zn^2
    let zr = current.x * current.x - current.y * current.y; // Real component
    let zi = 2.0 * current.x * current.y; // Imaginary component

    // Add constant
    Vec2 { x: zr, y: zi } + constant
}

// Returns the mod squared
pub fn mod2(z: Vec2) -> f32 {
    z.x * z.x + z.y * z.y
}

// Compute sequence of elements until mod exceeds threshold or max iterations is reached
pub fn compute_iterations(z0: Vec2, constant: Vec2, max_iterations: u32) -> u32 {
    let mut zn = z0;
    let mut iterations: u32 = 0;

    while mod2(zn) < 4.0 && iterations < max_iterations {
        zn = compute_next(zn, constant);
        iterations += 1;
    }

    iterations
}

// Compute sequence of elements until mod exceeds threshold or max iterations is reached
pub fn compute_iterations_smooth(z0: Vec2, constant: Vec2, max_iterations: u32) -> f32 {
    let mut zn = z0;
    let mut iterations: u32 = 0;

    while mod2(zn) < 4.0 && iterations < max_iterations {
        zn = compute_next(zn, constant);
        iterations += 1;
    }

    let modulo = mod2(zn).sqrt();
    let smooth_iteration = iterations as f32 - modulo.log2().max(1.0).log2();
    smooth_iteration
}

// Computes pixel color
pub fn draw(buffer: &mut Vec<u32>, viewport_size: Vec2, constant: Vec2, scale_in: f32) {
    // Compute the scale of the coordinates
    let scale = 1.0 / ((viewport_size.y / 2.0) + scale_in);

    for y in 0..viewport_size.y as i32 {
        for x in 0..viewport_size.x as i32 {
            // Compute pixel's coordinates
            let px = (x as f32 - viewport_size.x / 2.0) * scale;
            let py = (y as f32 - viewport_size.y / 2.0) * scale;

            // Compute color
            let iterations =
                compute_iterations_smooth(Vec2 { x: px, y: py }, constant, MAX_ITERATIONS);
            let buffer_index = x as usize + y as usize * viewport_size.x as usize;
            let color: u8 = (255.0 / MAX_ITERATIONS as f32 * iterations) as u8;

            buffer[buffer_index] = to_argb8(255, color, color, color);
        }
    }
}
