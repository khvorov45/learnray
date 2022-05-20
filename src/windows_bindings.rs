#![allow(dead_code, non_camel_case_types, non_snake_case)]

#[link(name = "Kernel32")]
extern "system" {
    pub fn ExitProcess(code: i32);
    pub fn DebugBreak();
}

#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassExW(param0: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dwexstyle: WINDOW_EX_STYLE,
        lpclassname: PCWSTR,
        lpwindowname: PCWSTR,
        dwstyle: WINDOW_STYLE,
        x: i32,
        y: i32,
        nwidth: i32,
        nheight: i32,
        hwndparent: HWND,
        hmenu: HMENU,
        hinstance: HINSTANCE,
        lpparam: *const ::core::ffi::c_void,
    ) -> HWND;
    pub fn GetModuleHandleExW(dwflags: u32, lpmodulename: PCWSTR, phmodule: *mut HINSTANCE)
        -> BOOL;
    pub fn DefWindowProcW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    pub fn ShowWindow(hwnd: HWND, ncmdshow: SHOW_WINDOW_CMD) -> BOOL;
}

pub type WINDOW_EX_STYLE = u32;
pub type WNDCLASS_STYLES = u32;

pub const WS_EX_APPWINDOW: WINDOW_EX_STYLE = 262144u32;

pub const WS_OVERLAPPED: WINDOW_STYLE = 0u32;
pub const WS_POPUP: WINDOW_STYLE = 2147483648u32;
pub const WS_CHILD: WINDOW_STYLE = 1073741824u32;
pub const WS_MINIMIZE: WINDOW_STYLE = 536870912u32;
pub const WS_VISIBLE: WINDOW_STYLE = 268435456u32;
pub const WS_DISABLED: WINDOW_STYLE = 134217728u32;
pub const WS_CLIPSIBLINGS: WINDOW_STYLE = 67108864u32;
pub const WS_CLIPCHILDREN: WINDOW_STYLE = 33554432u32;
pub const WS_MAXIMIZE: WINDOW_STYLE = 16777216u32;
pub const WS_CAPTION: WINDOW_STYLE = 12582912u32;
pub const WS_BORDER: WINDOW_STYLE = 8388608u32;
pub const WS_DLGFRAME: WINDOW_STYLE = 4194304u32;
pub const WS_VSCROLL: WINDOW_STYLE = 2097152u32;
pub const WS_HSCROLL: WINDOW_STYLE = 1048576u32;
pub const WS_SYSMENU: WINDOW_STYLE = 524288u32;
pub const WS_THICKFRAME: WINDOW_STYLE = 262144u32;
pub const WS_GROUP: WINDOW_STYLE = 131072u32;
pub const WS_TABSTOP: WINDOW_STYLE = 65536u32;
pub const WS_MINIMIZEBOX: WINDOW_STYLE = 131072u32;
pub const WS_MAXIMIZEBOX: WINDOW_STYLE = 65536u32;
pub const WS_TILED: WINDOW_STYLE = 0u32;
pub const WS_ICONIC: WINDOW_STYLE = 536870912u32;
pub const WS_SIZEBOX: WINDOW_STYLE = 262144u32;
pub const WS_TILEDWINDOW: WINDOW_STYLE = 13565952u32;
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = 13565952u32;
pub const WS_POPUPWINDOW: WINDOW_STYLE = 2156396544u32;
pub const WS_CHILDWINDOW: WINDOW_STYLE = 1073741824u32;
pub const WS_ACTIVECAPTION: WINDOW_STYLE = 1u32;

pub const CW_USEDEFAULT: i32 = -2147483648i32;

pub const SHOW_FULLSCREEN: u32 = 3u32;
pub const SHOW_ICONWINDOW: u32 = 2u32;
pub const SHOW_OPENNOACTIVATE: u32 = 4u32;
pub const SHOW_OPENWINDOW: u32 = 1u32;
pub type SHOW_WINDOW_CMD = u32;
pub const SW_FORCEMINIMIZE: SHOW_WINDOW_CMD = 11u32;
pub const SW_HIDE: SHOW_WINDOW_CMD = 0u32;
pub const SW_MAXIMIZE: SHOW_WINDOW_CMD = 3u32;
pub const SW_MINIMIZE: SHOW_WINDOW_CMD = 6u32;
pub const SW_RESTORE: SHOW_WINDOW_CMD = 9u32;
pub const SW_SHOW: SHOW_WINDOW_CMD = 5u32;
pub const SW_SHOWDEFAULT: SHOW_WINDOW_CMD = 10u32;
pub const SW_SHOWMAXIMIZED: SHOW_WINDOW_CMD = 3u32;
pub const SW_SHOWMINIMIZED: SHOW_WINDOW_CMD = 2u32;
pub const SW_SHOWMINNOACTIVE: SHOW_WINDOW_CMD = 7u32;
pub const SW_SHOWNA: SHOW_WINDOW_CMD = 8u32;
pub const SW_SHOWNOACTIVATE: SHOW_WINDOW_CMD = 4u32;
pub const SW_SHOWNORMAL: SHOW_WINDOW_CMD = 1u32;
pub const SW_NORMAL: SHOW_WINDOW_CMD = 1u32;
pub const SW_MAX: SHOW_WINDOW_CMD = 11u32;
pub const SW_PARENTCLOSING: SHOW_WINDOW_CMD = 1u32;
pub const SW_OTHERZOOM: SHOW_WINDOW_CMD = 2u32;
pub const SW_PARENTOPENING: SHOW_WINDOW_CMD = 3u32;
pub const SW_OTHERUNZOOM: SHOW_WINDOW_CMD = 4u32;
pub const SW_SCROLLCHILDREN: SHOW_WINDOW_CMD = 1u32;
pub const SW_INVALIDATE: SHOW_WINDOW_CMD = 2u32;
pub const SW_ERASE: SHOW_WINDOW_CMD = 4u32;
pub const SW_SMOOTHSCROLL: SHOW_WINDOW_CMD = 16u32;

pub type HRESULT = i32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type WINDOW_STYLE = u32;
pub type HWND = isize;
pub type HMENU = isize;
pub type HINSTANCE = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HICON = isize;
pub type HBITMAP = isize;
pub type HBRUSH = isize;
pub type HCURSOR = isize;
pub type HDC = isize;

pub type BOOL = i32;
pub type BOOLEAN = u8;
pub type BSTR = *mut u16;

pub type WNDPROC =
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT;

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: PCWSTR,
    pub lpszClassName: PCWSTR,
    pub hIconSm: HICON,
}
