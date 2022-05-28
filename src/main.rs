#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

#[macro_use]
mod log;

mod learnray;
mod math;
mod mem;
mod renderer;
mod window;

#[cfg(target_os = "windows")]
mod windows_bindings;

#[cfg(target_os = "windows")]
#[macro_use]
mod utf16;

#[cfg(target_os = "windows")]
mod log_windows;
#[cfg(target_os = "windows")]
use log_windows as platform_log;

#[cfg(target_os = "windows")]
mod window_windows;
#[cfg(target_os = "windows")]
use window_windows as platform_window;

#[cfg(target_os = "windows")]
mod mem_windows;
#[cfg(target_os = "windows")]
use mem_windows as platform_mem;

//
// SECTION Panic handling
//

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        log_debug!("{}", info);
        windows_bindings::DebugBreak();
        windows_bindings::ExitProcess(1);
    }
    loop {}
}

//
// SECTION Entry points
//

// NOTE(khvorov) Rust envokes the linker with /ENTRY:mainCRTStartup no matter what the subsystem is
#[cfg(target_os = "windows")]
#[no_mangle]
extern "C" fn mainCRTStartup() {
    learnray::main();
    unsafe {
        windows_bindings::ExitProcess(0);
    }
}

//
// SECTION CRT
//

#[no_mangle]
static _fltused: i32 = 0x9875;

#[no_mangle]
pub extern "C" fn __CxxFrameHandler3() {
    panic!("__CxxFrameHandler3")
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(_mem1: *const u8, _mem2: *const u8, _n: usize) -> i32 {
    panic!("memcmp")
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, val: i32, length: usize) -> *mut u8 {
    let mem_slice = core::slice::from_raw_parts_mut(mem, length);
    for byte in mem_slice {
        *byte = val as u8;
    }
    mem
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, length: usize) -> *mut u8 {
    let src_slice = core::slice::from_raw_parts(src, length);
    let dest_slice = core::slice::from_raw_parts_mut(dest, length);
    for index in 0..length {
        dest_slice[index] = src_slice[index]
    }
    dest
}
