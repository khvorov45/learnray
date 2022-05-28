use crate::math::V2i;

#[derive(Default)]
pub struct Window {
    pub is_running: bool,
    pub dim: V2i,
    pub platform: crate::platform_window::PlatformWindow,
}

impl Window {
    pub fn init(&mut self, width: i32, height: i32) {
        crate::platform_window::init(self, width, height);
    }
    pub fn poll_for_input(&mut self) {
        crate::platform_window::poll_for_input(self);
    }
    pub fn display_pixels(&mut self, pixels: *const [u32], px_width: i32, px_height: i32) {
        crate::platform_window::display_pixels(self, pixels, px_width, px_height);
    }
}
