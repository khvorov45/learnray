pub fn main() {
    let mut window = crate::window::Window::default();
    window.init();

    while window.is_running {
        window.poll_for_input();
    }
}
