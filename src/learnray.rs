use crate::math;
use crate::math::Color;
use crate::math::Ray;
use crate::math::Rect2i;
use crate::math::Sphere;
use crate::math::V2i;
use crate::math::V3;
use crate::mem;
use crate::mem::FixedArray;

struct Camera {
    pub eye: V3,
    pub viewport_lower_left: V3,
    pub viewport_h: V3,
    pub viewport_v: V3,
}

impl Camera {
    pub fn new(width_over_height: f32) -> Camera {
        let eye = V3::new(0.0, 0.0, 0.0);
        let camera_to_viewport: f32 = 1.0;

        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = width_over_height * viewport_height;

        let viewport_h = V3::new(viewport_width, 0.0, 0.0);
        let viewport_v = V3::new(0.0, viewport_height, 0.0);

        let viewport_lower_left =
            eye - viewport_h * 0.5 - viewport_v * 0.5 + V3::new(0.0, 0.0, -camera_to_viewport);

        Camera {
            eye,
            viewport_lower_left,
            viewport_h,
            viewport_v,
        }
    }
}

pub fn main() {
    let mut virtual_arena = mem::VirtualArena::new(1 * mem::GIGABYTE, 1 * mem::MEGABYTE);

    let mut window = crate::window::Window::default();
    window.init(V2i::new(1280, 720));

    let mut renderer = crate::renderer::Renderer::new(window.dim, &mut virtual_arena);

    let clear_color = Color::new(0.1, 0.1, 0.1, 1.0);

    let mut spheres = if let Ok(spheres) = FixedArray::new(2, &mut virtual_arena) {
        spheres
    } else {
        panic!("did not allocate spheres");
    };

    spheres[0] = Sphere::new(V3::new(0.0, 0.0, -1.0), 0.5);
    spheres[1] = Sphere::new(V3::new(0.0, -100.5, -1.0), 100.0);

    let mut rng = math::Lcg64Xsh32::default();

    let camera = Camera::new(window.dim.x as f32 / window.dim.y as f32);

    while window.is_running {
        //
        // SECTION Input
        //

        window.poll_for_input();

        //
        // SECTION Update
        //

        //
        // SECTION Render
        //

        renderer.clear_buffers(clear_color);

        for row in 0..renderer.dim.y {
            for col in 0..renderer.dim.x {
                let u = col as f32 / (renderer.dim.x - 1) as f32;
                let v = row as f32 / (renderer.dim.y - 1) as f32;

                let ray_dir =
                    camera.viewport_lower_left + u * camera.viewport_h + v * camera.viewport_v;
                let ray = Ray {
                    origin: camera.eye,
                    dir: ray_dir,
                };

                let _rand = rng.f32_01();

                let ray_color = get_ray_color(ray, &spheres);
                let ray_color32 = ray_color.to_u32argb();
                let px_index = ((renderer.dim.y - 1 - row) * renderer.dim.x + col) as usize;
                renderer.pixels[px_index] = ray_color32;
            }
        }

        renderer.draw_rect(
            Rect2i::new(V2i::new(50, 50), V2i::new(50, 50)),
            Color::new(0.0, 0.0, 1.0, 1.0),
        );

        window.display_pixels(&renderer.pixels, renderer.dim.x, renderer.dim.y);
    }
}

fn get_ray_color(ray: Ray, spheres: &[Sphere]) -> Color {
    let mut hit_color: Option<Color> = None;
    for sphere in spheres {
        if let Some(rec) = sphere.hit(ray, 0.0, 100.0) {
            hit_color = Some(Color::from_v3(0.5 * (rec.normal + 1.0)));
            break;
        }
    }

    if let Some(hit_color) = hit_color {
        hit_color
    } else {
        let dir_norm = ray.dir.normalize();
        let from_bottom = 0.5 * (dir_norm.y + 1.0);
        let bottom_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let top_color = Color::new(0.5, 0.7, 1.0, 1.0);
        (1.0 - from_bottom) * bottom_color + from_bottom * top_color
    }
}
