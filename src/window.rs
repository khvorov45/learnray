use crate::math::V2i;

#[derive(Default)]
pub struct Window {
    pub is_running: bool,
    pub dim: V2i,
    pub platform: crate::platform_window::PlatformWindow,
}
