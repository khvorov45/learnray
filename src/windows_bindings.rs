#![allow(dead_code, non_camel_case_types, non_snake_case)]

#[link(name = "Kernel32")]
extern "system" {
    pub fn ExitProcess(code: i32);
    pub fn DebugBreak();
    pub fn VirtualAlloc(
        lpaddress: *const ::core::ffi::c_void,
        dwsize: usize,
        flallocationtype: VIRTUAL_ALLOCATION_TYPE,
        flprotect: PAGE_PROTECTION_FLAGS,
    ) -> *mut ::core::ffi::c_void;
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
    pub fn LoadIconA(hinstance: HINSTANCE, lpiconname: PCSTR) -> HICON;
    pub fn LoadIconW(hinstance: HINSTANCE, lpiconname: PCWSTR) -> HICON;
    pub fn LoadCursorW(hinstance: HINSTANCE, lpcursorname: PCWSTR) -> HCURSOR;
    pub fn SetWindowLongPtrW(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
    pub fn GetWindowLongPtrW(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
    pub fn GetMessageW(lpmsg: *mut MSG, hwnd: HWND, wmsgfiltermin: u32, wmsgfiltermax: u32)
        -> BOOL;
    pub fn PeekMessageW(
        lpmsg: *mut MSG,
        hwnd: HWND,
        wmsgfiltermin: u32,
        wmsgfiltermax: u32,
        wremovemsg: PEEK_MESSAGE_REMOVE_TYPE,
    ) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
}

#[link(name = "Gdi32")]
extern "system" {
    pub fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ;
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

pub const CS_VREDRAW: WNDCLASS_STYLES = 1u32;
pub const CS_HREDRAW: WNDCLASS_STYLES = 2u32;
pub const CS_DBLCLKS: WNDCLASS_STYLES = 8u32;
pub const CS_OWNDC: WNDCLASS_STYLES = 32u32;
pub const CS_CLASSDC: WNDCLASS_STYLES = 64u32;
pub const CS_PARENTDC: WNDCLASS_STYLES = 128u32;
pub const CS_NOCLOSE: WNDCLASS_STYLES = 512u32;
pub const CS_SAVEBITS: WNDCLASS_STYLES = 2048u32;
pub const CS_BYTEALIGNCLIENT: WNDCLASS_STYLES = 4096u32;
pub const CS_BYTEALIGNWINDOW: WNDCLASS_STYLES = 8192u32;
pub const CS_GLOBALCLASS: WNDCLASS_STYLES = 16384u32;
pub const CS_IME: WNDCLASS_STYLES = 65536u32;
pub const CS_DROPSHADOW: WNDCLASS_STYLES = 131072u32;

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
pub type HENHMETAFILE = isize;
pub type HFONT = isize;
pub type HGDIOBJ = isize;
pub type HMETAFILE = isize;
pub type HMONITOR = isize;
pub type HPALETTE = isize;
pub type HPEN = isize;
pub type HRGN = isize;

pub type BOOL = i32;
pub type BOOLEAN = u8;
pub type BSTR = *mut u16;

pub const ICON_BIG: u32 = 1u32;
pub const ICON_SMALL: u32 = 0u32;
pub const ICON_SMALL2: u32 = 2u32;
pub const IDANI_CAPTION: u32 = 3u32;
pub const IDANI_OPEN: u32 = 1u32;
pub const IDC_APPSTARTING: PCWSTR = 32650i32 as _;
pub const IDC_ARROW: PCWSTR = 32512i32 as _;
pub const IDC_CROSS: PCWSTR = 32515i32 as _;
pub const IDC_HAND: PCWSTR = 32649i32 as _;
pub const IDC_HELP: PCWSTR = 32651i32 as _;
pub const IDC_IBEAM: PCWSTR = 32513i32 as _;
pub const IDC_ICON: PCWSTR = 32641i32 as _;
pub const IDC_NO: PCWSTR = 32648i32 as _;
pub const IDC_PERSON: PCWSTR = 32672i32 as _;
pub const IDC_PIN: PCWSTR = 32671i32 as _;
pub const IDC_SIZE: PCWSTR = 32640i32 as _;
pub const IDC_SIZEALL: PCWSTR = 32646i32 as _;
pub const IDC_SIZENESW: PCWSTR = 32643i32 as _;
pub const IDC_SIZENS: PCWSTR = 32645i32 as _;
pub const IDC_SIZENWSE: PCWSTR = 32642i32 as _;
pub const IDC_SIZEWE: PCWSTR = 32644i32 as _;
pub const IDC_UPARROW: PCWSTR = 32516i32 as _;
pub const IDC_WAIT: PCWSTR = 32514i32 as _;
pub const IDHOT_SNAPDESKTOP: i32 = -2i32;
pub const IDHOT_SNAPWINDOW: i32 = -1i32;
pub const IDH_CANCEL: u32 = 28444u32;
pub const IDH_GENERIC_HELP_BUTTON: u32 = 28442u32;
pub const IDH_HELP: u32 = 28445u32;
pub const IDH_MISSING_CONTEXT: u32 = 28441u32;
pub const IDH_NO_HELP: u32 = 28440u32;
pub const IDH_OK: u32 = 28443u32;
pub const IDI_APPLICATION: PCWSTR = 32512u32 as _;
pub const IDI_ASTERISK: PCWSTR = 32516u32 as _;
pub const IDI_ERROR: u32 = 32513u32;
pub const IDI_EXCLAMATION: PCWSTR = 32515u32 as _;
pub const IDI_HAND: PCWSTR = 32513u32 as _;
pub const IDI_INFORMATION: u32 = 32516u32;
pub const IDI_QUESTION: PCWSTR = 32514u32 as _;
pub const IDI_SHIELD: PCWSTR = 32518u32 as _;
pub const IDI_WARNING: u32 = 32515u32;
pub const IDI_WINLOGO: PCWSTR = 32517u32 as _;

pub type GET_STOCK_OBJECT_FLAGS = u32;
pub const BLACK_BRUSH: GET_STOCK_OBJECT_FLAGS = 4u32;
pub const DKGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 3u32;
pub const DC_BRUSH: GET_STOCK_OBJECT_FLAGS = 18u32;
pub const GRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 2u32;
pub const HOLLOW_BRUSH: GET_STOCK_OBJECT_FLAGS = 5u32;
pub const LTGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 1u32;
pub const NULL_BRUSH: GET_STOCK_OBJECT_FLAGS = 5u32;
pub const WHITE_BRUSH: GET_STOCK_OBJECT_FLAGS = 0u32;
pub const BLACK_PEN: GET_STOCK_OBJECT_FLAGS = 7u32;
pub const DC_PEN: GET_STOCK_OBJECT_FLAGS = 19u32;
pub const NULL_PEN: GET_STOCK_OBJECT_FLAGS = 8u32;
pub const WHITE_PEN: GET_STOCK_OBJECT_FLAGS = 6u32;
pub const ANSI_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 11u32;
pub const ANSI_VAR_FONT: GET_STOCK_OBJECT_FLAGS = 12u32;
pub const DEVICE_DEFAULT_FONT: GET_STOCK_OBJECT_FLAGS = 14u32;
pub const DEFAULT_GUI_FONT: GET_STOCK_OBJECT_FLAGS = 17u32;
pub const OEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 10u32;
pub const SYSTEM_FONT: GET_STOCK_OBJECT_FLAGS = 13u32;
pub const SYSTEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 16u32;
pub const DEFAULT_PALETTE: GET_STOCK_OBJECT_FLAGS = 15u32;
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;

pub type PEEK_MESSAGE_REMOVE_TYPE = u32;
pub const PM_NOREMOVE: PEEK_MESSAGE_REMOVE_TYPE = 0u32;
pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = 1u32;
pub const PM_NOYIELD: PEEK_MESSAGE_REMOVE_TYPE = 2u32;

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

#[derive(Default)]
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[derive(Default)]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

pub type WINDOW_LONG_PTR_INDEX = i32;
pub const GWL_EXSTYLE: WINDOW_LONG_PTR_INDEX = -20i32;
pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = -6i32;
pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = -8i32;
pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = -12i32;
pub const GWL_STYLE: WINDOW_LONG_PTR_INDEX = -16i32;
pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = -21i32;
pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = -4i32;
pub const GWL_HINSTANCE: WINDOW_LONG_PTR_INDEX = -6i32;
pub const GWL_ID: WINDOW_LONG_PTR_INDEX = -12i32;
pub const GWL_USERDATA: WINDOW_LONG_PTR_INDEX = -21i32;
pub const GWL_WNDPROC: WINDOW_LONG_PTR_INDEX = -4i32;
pub const GWL_HWNDPARENT: WINDOW_LONG_PTR_INDEX = -8i32;

pub const WMSZ_BOTTOM: u32 = 6u32;
pub const WMSZ_BOTTOMLEFT: u32 = 7u32;
pub const WMSZ_BOTTOMRIGHT: u32 = 8u32;
pub const WMSZ_LEFT: u32 = 1u32;
pub const WMSZ_RIGHT: u32 = 2u32;
pub const WMSZ_TOP: u32 = 3u32;
pub const WMSZ_TOPLEFT: u32 = 4u32;
pub const WMSZ_TOPRIGHT: u32 = 5u32;
pub const WM_ACTIVATE: u32 = 6u32;
pub const WM_ACTIVATEAPP: u32 = 28u32;
pub const WM_AFXFIRST: u32 = 864u32;
pub const WM_AFXLAST: u32 = 895u32;
pub const WM_APP: u32 = 32768u32;
pub const WM_APPCOMMAND: u32 = 793u32;
pub const WM_ASKCBFORMATNAME: u32 = 780u32;
pub const WM_CANCELJOURNAL: u32 = 75u32;
pub const WM_CANCELMODE: u32 = 31u32;
pub const WM_CAPTURECHANGED: u32 = 533u32;
pub const WM_CHANGECBCHAIN: u32 = 781u32;
pub const WM_CHANGEUISTATE: u32 = 295u32;
pub const WM_CHAR: u32 = 258u32;
pub const WM_CHARTOITEM: u32 = 47u32;
pub const WM_CHILDACTIVATE: u32 = 34u32;
pub const WM_CLEAR: u32 = 771u32;
pub const WM_CLIPBOARDUPDATE: u32 = 797u32;
pub const WM_CLOSE: u32 = 16u32;
pub const WM_COMMAND: u32 = 273u32;
pub const WM_COMMNOTIFY: u32 = 68u32;
pub const WM_COMPACTING: u32 = 65u32;
pub const WM_COMPAREITEM: u32 = 57u32;
pub const WM_CONTEXTMENU: u32 = 123u32;
pub const WM_COPY: u32 = 769u32;
pub const WM_COPYDATA: u32 = 74u32;
pub const WM_CREATE: u32 = 1u32;
pub const WM_CTLCOLORBTN: u32 = 309u32;
pub const WM_CTLCOLORDLG: u32 = 310u32;
pub const WM_CTLCOLOREDIT: u32 = 307u32;
pub const WM_CTLCOLORLISTBOX: u32 = 308u32;
pub const WM_CTLCOLORMSGBOX: u32 = 306u32;
pub const WM_CTLCOLORSCROLLBAR: u32 = 311u32;
pub const WM_CTLCOLORSTATIC: u32 = 312u32;
pub const WM_CUT: u32 = 768u32;
pub const WM_DEADCHAR: u32 = 259u32;
pub const WM_DELETEITEM: u32 = 45u32;
pub const WM_DESTROY: u32 = 2u32;
pub const WM_DESTROYCLIPBOARD: u32 = 775u32;
pub const WM_DEVICECHANGE: u32 = 537u32;
pub const WM_DEVMODECHANGE: u32 = 27u32;
pub const WM_DISPLAYCHANGE: u32 = 126u32;
pub const WM_DPICHANGED: u32 = 736u32;
pub const WM_DPICHANGED_AFTERPARENT: u32 = 739u32;
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 738u32;
pub const WM_DRAWCLIPBOARD: u32 = 776u32;
pub const WM_DRAWITEM: u32 = 43u32;
pub const WM_DROPFILES: u32 = 563u32;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 800u32;
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 798u32;
pub const WM_DWMNCRENDERINGCHANGED: u32 = 799u32;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 806u32;
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 803u32;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 801u32;
pub const WM_ENABLE: u32 = 10u32;
pub const WM_ENDSESSION: u32 = 22u32;
pub const WM_ENTERIDLE: u32 = 289u32;
pub const WM_ENTERMENULOOP: u32 = 529u32;
pub const WM_ENTERSIZEMOVE: u32 = 561u32;
pub const WM_ERASEBKGND: u32 = 20u32;
pub const WM_EXITMENULOOP: u32 = 530u32;
pub const WM_EXITSIZEMOVE: u32 = 562u32;
pub const WM_FONTCHANGE: u32 = 29u32;
pub const WM_GESTURE: u32 = 281u32;
pub const WM_GESTURENOTIFY: u32 = 282u32;
pub const WM_GETDLGCODE: u32 = 135u32;
pub const WM_GETDPISCALEDSIZE: u32 = 740u32;
pub const WM_GETFONT: u32 = 49u32;
pub const WM_GETHOTKEY: u32 = 51u32;
pub const WM_GETICON: u32 = 127u32;
pub const WM_GETMINMAXINFO: u32 = 36u32;
pub const WM_GETOBJECT: u32 = 61u32;
pub const WM_GETTEXT: u32 = 13u32;
pub const WM_GETTEXTLENGTH: u32 = 14u32;
pub const WM_GETTITLEBARINFOEX: u32 = 831u32;
pub const WM_HANDHELDFIRST: u32 = 856u32;
pub const WM_HANDHELDLAST: u32 = 863u32;
pub const WM_HELP: u32 = 83u32;
pub const WM_HOTKEY: u32 = 786u32;
pub const WM_HSCROLL: u32 = 276u32;
pub const WM_HSCROLLCLIPBOARD: u32 = 782u32;
pub const WM_ICONERASEBKGND: u32 = 39u32;
pub const WM_IME_CHAR: u32 = 646u32;
pub const WM_IME_COMPOSITION: u32 = 271u32;
pub const WM_IME_COMPOSITIONFULL: u32 = 644u32;
pub const WM_IME_CONTROL: u32 = 643u32;
pub const WM_IME_ENDCOMPOSITION: u32 = 270u32;
pub const WM_IME_KEYDOWN: u32 = 656u32;
pub const WM_IME_KEYLAST: u32 = 271u32;
pub const WM_IME_KEYUP: u32 = 657u32;
pub const WM_IME_NOTIFY: u32 = 642u32;
pub const WM_IME_REQUEST: u32 = 648u32;
pub const WM_IME_SELECT: u32 = 645u32;
pub const WM_IME_SETCONTEXT: u32 = 641u32;
pub const WM_IME_STARTCOMPOSITION: u32 = 269u32;
pub const WM_INITDIALOG: u32 = 272u32;
pub const WM_INITMENU: u32 = 278u32;
pub const WM_INITMENUPOPUP: u32 = 279u32;
pub const WM_INPUT: u32 = 255u32;
pub const WM_INPUTLANGCHANGE: u32 = 81u32;
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 80u32;
pub const WM_INPUT_DEVICE_CHANGE: u32 = 254u32;
pub const WM_KEYDOWN: u32 = 256u32;
pub const WM_KEYFIRST: u32 = 256u32;
pub const WM_KEYLAST: u32 = 265u32;
pub const WM_KEYUP: u32 = 257u32;
pub const WM_KILLFOCUS: u32 = 8u32;
pub const WM_LBUTTONDBLCLK: u32 = 515u32;
pub const WM_LBUTTONDOWN: u32 = 513u32;
pub const WM_LBUTTONUP: u32 = 514u32;
pub const WM_MBUTTONDBLCLK: u32 = 521u32;
pub const WM_MBUTTONDOWN: u32 = 519u32;
pub const WM_MBUTTONUP: u32 = 520u32;
pub const WM_MDIACTIVATE: u32 = 546u32;
pub const WM_MDICASCADE: u32 = 551u32;
pub const WM_MDICREATE: u32 = 544u32;
pub const WM_MDIDESTROY: u32 = 545u32;
pub const WM_MDIGETACTIVE: u32 = 553u32;
pub const WM_MDIICONARRANGE: u32 = 552u32;
pub const WM_MDIMAXIMIZE: u32 = 549u32;
pub const WM_MDINEXT: u32 = 548u32;
pub const WM_MDIREFRESHMENU: u32 = 564u32;
pub const WM_MDIRESTORE: u32 = 547u32;
pub const WM_MDISETMENU: u32 = 560u32;
pub const WM_MDITILE: u32 = 550u32;
pub const WM_MEASUREITEM: u32 = 44u32;
pub const WM_MENUCHAR: u32 = 288u32;
pub const WM_MENUCOMMAND: u32 = 294u32;
pub const WM_MENUDRAG: u32 = 291u32;
pub const WM_MENUGETOBJECT: u32 = 292u32;
pub const WM_MENURBUTTONUP: u32 = 290u32;
pub const WM_MENUSELECT: u32 = 287u32;
pub const WM_MOUSEACTIVATE: u32 = 33u32;
pub const WM_MOUSEFIRST: u32 = 512u32;
pub const WM_MOUSEHWHEEL: u32 = 526u32;
pub const WM_MOUSELAST: u32 = 526u32;
pub const WM_MOUSEMOVE: u32 = 512u32;
pub const WM_MOUSEWHEEL: u32 = 522u32;
pub const WM_MOVE: u32 = 3u32;
pub const WM_MOVING: u32 = 534u32;
pub const WM_NCACTIVATE: u32 = 134u32;
pub const WM_NCCALCSIZE: u32 = 131u32;
pub const WM_NCCREATE: u32 = 129u32;
pub const WM_NCDESTROY: u32 = 130u32;
pub const WM_NCHITTEST: u32 = 132u32;
pub const WM_NCLBUTTONDBLCLK: u32 = 163u32;
pub const WM_NCLBUTTONDOWN: u32 = 161u32;
pub const WM_NCLBUTTONUP: u32 = 162u32;
pub const WM_NCMBUTTONDBLCLK: u32 = 169u32;
pub const WM_NCMBUTTONDOWN: u32 = 167u32;
pub const WM_NCMBUTTONUP: u32 = 168u32;
pub const WM_NCMOUSEHOVER: u32 = 672u32;
pub const WM_NCMOUSELEAVE: u32 = 674u32;
pub const WM_NCMOUSEMOVE: u32 = 160u32;
pub const WM_NCPAINT: u32 = 133u32;
pub const WM_NCPOINTERDOWN: u32 = 578u32;
pub const WM_NCPOINTERUP: u32 = 579u32;
pub const WM_NCPOINTERUPDATE: u32 = 577u32;
pub const WM_NCRBUTTONDBLCLK: u32 = 166u32;
pub const WM_NCRBUTTONDOWN: u32 = 164u32;
pub const WM_NCRBUTTONUP: u32 = 165u32;
pub const WM_NCXBUTTONDBLCLK: u32 = 173u32;
pub const WM_NCXBUTTONDOWN: u32 = 171u32;
pub const WM_NCXBUTTONUP: u32 = 172u32;
pub const WM_NEXTDLGCTL: u32 = 40u32;
pub const WM_NEXTMENU: u32 = 531u32;
pub const WM_NOTIFY: u32 = 78u32;
pub const WM_NOTIFYFORMAT: u32 = 85u32;
pub const WM_NULL: u32 = 0u32;
pub const WM_PAINT: u32 = 15u32;
pub const WM_PAINTCLIPBOARD: u32 = 777u32;
pub const WM_PAINTICON: u32 = 38u32;
pub const WM_PALETTECHANGED: u32 = 785u32;
pub const WM_PALETTEISCHANGING: u32 = 784u32;
pub const WM_PARENTNOTIFY: u32 = 528u32;
pub const WM_PASTE: u32 = 770u32;
pub const WM_PENWINFIRST: u32 = 896u32;
pub const WM_PENWINLAST: u32 = 911u32;
pub const WM_POINTERACTIVATE: u32 = 587u32;
pub const WM_POINTERCAPTURECHANGED: u32 = 588u32;
pub const WM_POINTERDEVICECHANGE: u32 = 568u32;
pub const WM_POINTERDEVICEINRANGE: u32 = 569u32;
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 570u32;
pub const WM_POINTERDOWN: u32 = 582u32;
pub const WM_POINTERENTER: u32 = 585u32;
pub const WM_POINTERHWHEEL: u32 = 591u32;
pub const WM_POINTERLEAVE: u32 = 586u32;
pub const WM_POINTERROUTEDAWAY: u32 = 594u32;
pub const WM_POINTERROUTEDRELEASED: u32 = 595u32;
pub const WM_POINTERROUTEDTO: u32 = 593u32;
pub const WM_POINTERUP: u32 = 583u32;
pub const WM_POINTERUPDATE: u32 = 581u32;
pub const WM_POINTERWHEEL: u32 = 590u32;
pub const WM_POWER: u32 = 72u32;
pub const WM_POWERBROADCAST: u32 = 536u32;
pub const WM_PRINT: u32 = 791u32;
pub const WM_PRINTCLIENT: u32 = 792u32;
pub const WM_QUERYDRAGICON: u32 = 55u32;
pub const WM_QUERYENDSESSION: u32 = 17u32;
pub const WM_QUERYNEWPALETTE: u32 = 783u32;
pub const WM_QUERYOPEN: u32 = 19u32;
pub const WM_QUERYUISTATE: u32 = 297u32;
pub const WM_QUEUESYNC: u32 = 35u32;
pub const WM_QUIT: u32 = 18u32;
pub const WM_RBUTTONDBLCLK: u32 = 518u32;
pub const WM_RBUTTONDOWN: u32 = 516u32;
pub const WM_RBUTTONUP: u32 = 517u32;
pub const WM_RENDERALLFORMATS: u32 = 774u32;
pub const WM_RENDERFORMAT: u32 = 773u32;
pub const WM_SETCURSOR: u32 = 32u32;
pub const WM_SETFOCUS: u32 = 7u32;
pub const WM_SETFONT: u32 = 48u32;
pub const WM_SETHOTKEY: u32 = 50u32;
pub const WM_SETICON: u32 = 128u32;
pub const WM_SETREDRAW: u32 = 11u32;
pub const WM_SETTEXT: u32 = 12u32;
pub const WM_SETTINGCHANGE: u32 = 26u32;
pub const WM_SHOWWINDOW: u32 = 24u32;
pub const WM_SIZE: u32 = 5u32;
pub const WM_SIZECLIPBOARD: u32 = 779u32;
pub const WM_SIZING: u32 = 532u32;
pub const WM_SPOOLERSTATUS: u32 = 42u32;
pub const WM_STYLECHANGED: u32 = 125u32;
pub const WM_STYLECHANGING: u32 = 124u32;
pub const WM_SYNCPAINT: u32 = 136u32;
pub const WM_SYSCHAR: u32 = 262u32;
pub const WM_SYSCOLORCHANGE: u32 = 21u32;
pub const WM_SYSCOMMAND: u32 = 274u32;
pub const WM_SYSDEADCHAR: u32 = 263u32;
pub const WM_SYSKEYDOWN: u32 = 260u32;
pub const WM_SYSKEYUP: u32 = 261u32;
pub const WM_TABLET_FIRST: u32 = 704u32;
pub const WM_TABLET_LAST: u32 = 735u32;
pub const WM_TCARD: u32 = 82u32;
pub const WM_THEMECHANGED: u32 = 794u32;
pub const WM_TIMECHANGE: u32 = 30u32;
pub const WM_TIMER: u32 = 275u32;
pub const WM_TOUCH: u32 = 576u32;
pub const WM_TOUCHHITTESTING: u32 = 589u32;
pub const WM_UNDO: u32 = 772u32;
pub const WM_UNICHAR: u32 = 265u32;
pub const WM_UNINITMENUPOPUP: u32 = 293u32;
pub const WM_UPDATEUISTATE: u32 = 296u32;
pub const WM_USER: u32 = 1024u32;
pub const WM_USERCHANGED: u32 = 84u32;
pub const WM_VKEYTOITEM: u32 = 46u32;
pub const WM_VSCROLL: u32 = 277u32;
pub const WM_VSCROLLCLIPBOARD: u32 = 778u32;
pub const WM_WINDOWPOSCHANGED: u32 = 71u32;
pub const WM_WINDOWPOSCHANGING: u32 = 70u32;
pub const WM_WININICHANGE: u32 = 26u32;
pub const WM_WTSSESSION_CHANGE: u32 = 689u32;
pub const WM_XBUTTONDBLCLK: u32 = 525u32;
pub const WM_XBUTTONDOWN: u32 = 523u32;
pub const WM_XBUTTONUP: u32 = 524u32;

pub type VIRTUAL_ALLOCATION_TYPE = u32;
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = 4096u32;
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = 8192u32;
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = 524288u32;
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = 16777216u32;
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 16384u32;
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = 536870912u32;
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 262144u32;
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = 65536u32;
pub type VIRTUAL_FREE_TYPE = u32;
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = 16384u32;
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = 32768u32;

pub type PAGE_PROTECTION_FLAGS = u32;
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = 1u32;
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = 2u32;
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = 4u32;
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = 8u32;
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = 16u32;
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32u32;
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 64u32;
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = 128u32;
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = 256u32;
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = 512u32;
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1024u32;
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = 2048u32;
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = 4096u32;
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = 8192u32;
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = 16384u32;
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32768u32;
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 65536u32;
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = 131072u32;
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = 262144u32;
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = 536870912u32;
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = 268435457u32;
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = 268435458u32;
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = 262144u32;
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = 524288u32;
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = 8388608u32;
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = 16777216u32;
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = 33554432u32;
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = 67108864u32;
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = 134217728u32;
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = 285212672u32;
pub type PAGE_TYPE = u32;
pub const MEM_PRIVATE: PAGE_TYPE = 131072u32;
pub const MEM_MAPPED: PAGE_TYPE = 262144u32;
pub const MEM_IMAGE: PAGE_TYPE = 16777216u32;
