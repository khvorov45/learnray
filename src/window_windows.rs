use crate::windows_bindings as win;

#[derive(Default)]
pub struct PlatformWindow {
    hwnd: win::HWND,
}

pub fn init(window: &mut crate::window::Window) {
    let mut window_class = win::WNDCLASSEXW {
        cbSize: core::mem::size_of::<win::WNDCLASSEXW>() as u32,
        style: 0,
        lpfnWndProc: window_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: 0,
        lpszMenuName: core::ptr::null(),
        lpszClassName: &utf16_null!("learnrayClassName")[0],
        hIconSm: 0,
    };

    unsafe {
        win::GetModuleHandleExW(0, core::ptr::null(), &mut window_class.hInstance);
        win::RegisterClassExW(&window_class);

        window.platform.hwnd = win::CreateWindowExW(
            win::WS_EX_APPWINDOW,
            window_class.lpszClassName,
            &utf16_null!("learnray")[0],
            win::WS_OVERLAPPEDWINDOW,
            win::CW_USEDEFAULT,
            win::CW_USEDEFAULT,
            1000,
            1000,
            0,
            0,
            window_class.hInstance,
            core::ptr::null(),
        );

        // NOTE(khvorov) To avoid a white flash
        win::ShowWindow(window.platform.hwnd, win::SW_SHOWMINIMIZED);
        win::ShowWindow(window.platform.hwnd, win::SW_SHOWNORMAL);
    }
}

unsafe extern "system" fn window_proc(
    hwnd: win::HWND,
    message: u32,
    wparam: win::WPARAM,
    lparam: win::LPARAM,
) -> win::LRESULT {
    let result;
    result = win::DefWindowProcW(hwnd, message, wparam, lparam);
    return result;
}
