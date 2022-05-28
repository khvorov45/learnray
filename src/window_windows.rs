use crate::math::V2i;
use crate::window::Window;
use crate::windows_bindings as win;

#[derive(Default)]
pub struct PlatformWindow {
    hdc: win::HDC,
    bmi: win::BITMAPINFO,
}

impl Window {
    pub fn init(&mut self, width: i32, height: i32) {
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

        let dim = V2i::new(width, height);

        let hdc = unsafe {
            win::GetModuleHandleExW(0, core::ptr::null(), &mut window_class.hInstance);
            win::RegisterClassExW(&window_class);

            let hwnd = win::CreateWindowExW(
                win::WS_EX_APPWINDOW,
                window_class.lpszClassName,
                &utf16_null!("learnray")[0],
                win::WS_OVERLAPPEDWINDOW,
                win::CW_USEDEFAULT,
                win::CW_USEDEFAULT,
                dim.x,
                dim.y,
                0,
                0,
                window_class.hInstance,
                core::ptr::null(),
            );

            win::SetWindowLongPtrW(hwnd, win::GWLP_USERDATA, self as *const Window as isize);

            // NOTE(khvorov) To avoid a white flash
            win::ShowWindow(hwnd, win::SW_SHOWMINIMIZED);
            win::ShowWindow(hwnd, win::SW_SHOWNORMAL);

            let hdc = win::GetDC(hwnd);

            hdc
        };

        // NOTE(khvorov) Set width and height before displaying pixels
        let mut bmi = win::BITMAPINFO::default();
        bmi.bmiHeader.biSize = core::mem::size_of::<win::BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = win::BI_RGB as u32;

        let platform = PlatformWindow { hdc, bmi };

        *self = Window {
            is_running: true,
            dim: dim,
            platform: platform,
        }
    }

    pub fn poll_for_input(&mut self) {
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

    pub fn display_pixels(&mut self, pixels: *const [u32], px_width: i32, px_height: i32) {
        self.platform.bmi.bmiHeader.biWidth = px_width;
        self.platform.bmi.bmiHeader.biHeight = -px_height; // NOTE(khvorov) Negative = top-down
        unsafe {
            win::StretchDIBits(
                self.platform.hdc,
                0,
                0,
                self.dim.x,
                self.dim.y,
                0,
                0,
                px_width,
                px_height,
                pixels as *const core::ffi::c_void,
                &self.platform.bmi,
                win::DIB_RGB_COLORS,
                win::SRCCOPY,
            );
        };
    }
}

unsafe extern "system" fn window_proc(
    hwnd: win::HWND,
    message: u32,
    wparam: win::WPARAM,
    lparam: win::LPARAM,
) -> win::LRESULT {
    let window_ptr = win::GetWindowLongPtrW(hwnd, win::GWLP_USERDATA);

    if window_ptr != 0 {
        let window = &mut *(window_ptr as *mut Window);

        match message {
            win::WM_CLOSE | win::WM_DESTROY => window.is_running = false,
            _ => {}
        }
    }

    win::DefWindowProcW(hwnd, message, wparam, lparam)
}
