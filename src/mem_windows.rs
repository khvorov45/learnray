use crate::windows_bindings as win;

pub fn reserve(size: usize) -> (*const u8, usize) {
    let ptr = unsafe {
        win::VirtualAlloc(
            core::ptr::null(),
            size,
            win::MEM_RESERVE,
            win::PAGE_READWRITE,
        ) as *const u8
    };
    (ptr, size)
}

pub fn commit(ptr: *const u8, size: usize) -> usize {
    unsafe {
        win::VirtualAlloc(
            ptr as *mut core::ffi::c_void,
            size,
            win::MEM_COMMIT,
            win::PAGE_READWRITE,
        )
    };
    size
}
