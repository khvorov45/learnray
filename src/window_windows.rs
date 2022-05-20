use crate::window::Window;
use crate::windows_bindings as win;

#[derive(Default)]
pub struct PlatformWindow {
    hwnd: win::HWND,
}

pub fn init(window: &mut Window) {
    let mut window_class = win::WNDCLASSEXW {
        cbSize: core::mem::size_of::<win::WNDCLASSEXW>() as u32,
        style: win::CS_VREDRAW | win::CS_HREDRAW,
        lpfnWndProc: window_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0,
        hIcon: unsafe { win::LoadIconW(0, win::IDI_APPLICATION) },
        hCursor: unsafe { win::LoadCursorW(0, win::IDC_ARROW) },
        hbrBackground: unsafe { win::GetStockObject(win::BLACK_BRUSH) } as win::HBRUSH,
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

        win::SetWindowLongPtrW(
            window.platform.hwnd,
            win::GWLP_USERDATA,
            window as *const Window as isize,
        );

        // NOTE(khvorov) To avoid a white flash
        win::ShowWindow(window.platform.hwnd, win::SW_SHOWMINIMIZED);
        win::ShowWindow(window.platform.hwnd, win::SW_SHOWNORMAL);

        window.is_running = true;
    }
}

unsafe extern "system" fn window_proc(
    hwnd: win::HWND,
    message: u32,
    wparam: win::WPARAM,
    lparam: win::LPARAM,
) -> win::LRESULT {
    let result;

    let window_ptr = win::GetWindowLongPtrW(hwnd, win::GWLP_USERDATA);

    if window_ptr != 0 {
        let window = &mut *(window_ptr as *mut Window);

        match message {
            win::WM_CLOSE | win::WM_DESTROY => window.is_running = false,
            _ => {}
        }
    }

    result = win::DefWindowProcW(hwnd, message, wparam, lparam);
    return result;
}

pub fn poll_for_input(_window: &mut Window) {
    let mut msg = win::MSG::default();
    while unsafe { win::PeekMessageW(&mut msg, 0, 0, 0, win::PM_REMOVE) } != 0 {
        match msg.message {
            _ => unsafe {
                win::TranslateMessage(&msg);
                win::DispatchMessageW(&msg);
            },
        }
    }
}
