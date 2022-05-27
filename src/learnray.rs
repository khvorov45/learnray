use crate::math::Color;
use crate::math::Ray;
use crate::math::V3;
use crate::mem;

pub fn main() {
    let mut virtual_arena = mem::VirtualArena::default();
    virtual_arena.init(1 * mem::GIGABYTE, 1 * mem::MEGABYTE);

    let mut window = crate::window::Window::default();
    window.init(1280, 720);

    let mut renderer = crate::renderer::Renderer::default();
    renderer.init(7680, 4320, &mut virtual_arena);

    let clear_color = Color {
        r: 0.1,
        g: 0.1,
        b: 0.1,
        a: 1.0,
    };

    while window.is_running {
        window.poll_for_input();

        renderer.clear_buffers(window.dim.x, window.dim.y, clear_color);

        let width_over_height = renderer.dim.x as f32 / window.dim.x as f32;

        let camera_p = V3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let camera_to_viewport: f32 = 1.0;

        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = width_over_height * viewport_height;

        let viewport_horizontal = V3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_vertical = V3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        let viewport_lower_left_corner =
            camera_p - viewport_horizontal * 0.5 - viewport_vertical * 0.5
                + V3 {
                    x: 0.0,
                    y: 0.0,
                    z: -camera_to_viewport,
                };

        for row in 0..renderer.dim.y {
            for col in 0..renderer.dim.x {
                let u = col as f32 / (renderer.dim.x - 1) as f32;
                let v = row as f32 / (renderer.dim.y - 1) as f32;

                let ray_dir =
                    viewport_lower_left_corner + u * viewport_horizontal + v * viewport_vertical;
                let ray = Ray {
                    orig: camera_p,
                    dir: ray_dir,
                };

                let ray_color = get_ray_color(ray);
                let ray_color32 = ray_color.to_u32argb();
                let px_index = ((renderer.dim.y - 1 - row) * renderer.dim.x + col) as usize;
                renderer.pixels[px_index] = ray_color32;
            }
        }

        window.display_pixels(renderer.pixels, renderer.dim.x, renderer.dim.y);
    }
}

fn get_ray_color(ray: Ray) -> Color {
    let dir_norm = ray.dir.normalize();
    let from_bottom = 0.5 * (dir_norm.y + 1.0);
    let bottom_color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    let top_color = Color {
        r: 0.5,
        g: 0.7,
        b: 1.0,
        a: 1.0,
    };
    (1.0 - from_bottom) * bottom_color + from_bottom * top_color
}
