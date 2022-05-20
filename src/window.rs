#[derive(Default)]
pub struct Window {
    pub platform: crate::platform_window::PlatformWindow,
}

impl Window {
    pub fn init(self: &mut Window) {
        crate::platform_window::init(self);
    }
}
