#[derive(Default)]
pub struct Window {
    pub is_running: bool,
    pub platform: crate::platform_window::PlatformWindow,
}

impl Window {
    pub fn init(self: &mut Window) {
        crate::platform_window::init(self);
    }
    pub fn poll_for_input(self: &mut Window) {
        crate::platform_window::poll_for_input(self);
    }
}
