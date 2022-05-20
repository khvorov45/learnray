#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

mod learnray;
mod window;

#[macro_use]
mod utf16;

#[cfg(target_os = "windows")]
mod windows_bindings;

#[cfg(target_os = "windows")]
mod window_windows;
#[cfg(target_os = "windows")]
use window_windows as platform_window;

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
