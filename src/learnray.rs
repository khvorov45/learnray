use crate::math::Color;
use crate::mem;

pub fn main() {
    let mut virtual_arena = mem::VirtualArena::default();
    virtual_arena.init(1 * mem::GIGABYTE, 1 * mem::MEGABYTE);

    let mut window = crate::window::Window::default();
    window.init();

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

        window.display_pixels(renderer.pixels, renderer.dim.x, renderer.dim.y);
    }
}
