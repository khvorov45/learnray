#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

mod learnray;
mod mem;
mod window;

#[macro_use]
mod utf16;

#[cfg(target_os = "windows")]
mod windows_bindings;

#[cfg(target_os = "windows")]
mod window_windows;
#[cfg(target_os = "windows")]
use window_windows as platform_window;

#[cfg(target_os = "windows")]
mod mem_windows;
#[cfg(target_os = "windows")]
use mem_windows as platform_mem;

// NOTE(khvorov) Not using built-in panic stuff
#[panic_handler]
fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "windows")]
#[no_mangle]
extern "C" fn mainCRTStartup() {
    learnray::main();
    unsafe {
        windows_bindings::ExitProcess(0);
    }
}

#[allow(dead_code)]
#[cfg(target_os = "windows")]
fn assert(cond: bool) {
    if !cond {
        unsafe {
            windows_bindings::DebugBreak();
        }
    }
}

//
// SECTION CRT
//

#[no_mangle]
static _fltused: i32 = 0x9875;

#[no_mangle]
pub extern "C" fn __CxxFrameHandler3() {}

#[no_mangle]
pub unsafe extern "C" fn memcmp(_mem1: *const u8, _mem2: *const u8, _n: usize) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, _val: i32, _n: usize) -> *mut u8 {
    mem
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    dest
}
