#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::fmt::Write;

mod learnray;
mod mem;
mod window;

#[cfg(target_os = "windows")]
mod windows_bindings;

#[cfg(target_os = "windows")]
#[macro_use]
mod utf16;

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

struct DebugOutput {
    buf: [u8; 1024],
    used: usize,
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self {
            buf: [0; 1024],
            used: 0,
        }
    }
}

impl Write for DebugOutput {
    fn write_str(&mut self, msg: &str) -> Result<(), core::fmt::Error> {
        let mut result = Ok(());
        for byte in msg.bytes() {
            if self.used < self.buf.len() - 1 {
                // NOTE(khvorov) Null terminator
                self.buf[self.used] = byte;
                self.used += 1;
            } else {
                result = Err(core::fmt::Error::default());
                break;
            }
        }
        result
    }
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let mut output = DebugOutput::default();
        if write!(&mut output, "{}", info).is_ok() {
            windows_bindings::OutputDebugStringA(&output.buf[0]);
        }
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
