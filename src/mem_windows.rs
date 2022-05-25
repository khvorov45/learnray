use crate::windows_bindings as win;

pub fn reserve(size: usize, ptr: &mut *const u8, size_actual: &mut usize) {
    *ptr = unsafe {
        win::VirtualAlloc(
            core::ptr::null(),
            size,
            win::MEM_RESERVE,
            win::PAGE_READWRITE,
        ) as *const u8
    };
    *size_actual = size;
}

pub fn commit(ptr: *const u8, size: usize, size_actual: &mut usize) {
    unsafe {
        win::VirtualAlloc(
            ptr as *mut core::ffi::c_void,
            size,
            win::MEM_COMMIT,
            win::PAGE_READWRITE,
        )
    };
    *size_actual = size;
}
