use core::mem::align_of;
use core::mem::size_of;

pub const BYTE: usize = 1;
pub const KILOBYTE: usize = 1024 * BYTE;
pub const MEGABYTE: usize = 1024 * KILOBYTE;
pub const GIGABYTE: usize = 1024 * MEGABYTE;

fn is_power_of_2(val: usize) -> bool {
    (val & (val - 1)) == 0
}

fn align_ptr(ptr: *const u8, align: usize, size: usize) -> (*const u8, usize) {
    assert!(is_power_of_2(align));
    let off_by = ptr as usize & (align - 1);
    let mut move_by = 0;
    if off_by > 0 {
        move_by = align - off_by;
    }
    let ptr_aligned = ptr as usize + move_by;
    let size_aligned = size + move_by;
    (ptr_aligned as *const u8, size_aligned)
}

pub enum Error {
    OutOfMemory,
}

pub trait Allocator {
    fn alloc(self: &mut Self, size: usize, align: usize) -> Result<*const u8, Error>;
}

pub struct FixedArray<T> {
    pub ptr: *mut T,
    pub len: usize,
}

impl<T> core::ops::Index<usize> for FixedArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        unsafe { &*((self.ptr as usize + index * size_of::<T>()) as *const T) }
    }
}

impl<T> core::ops::IndexMut<usize> for FixedArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut *((self.ptr as usize + index * size_of::<T>()) as *mut T) }
    }
}

impl<T> core::ops::Deref for FixedArray<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> Default for FixedArray<T> {
    fn default() -> FixedArray<T> {
        FixedArray {
            ptr: 0 as *mut T,
            len: 0,
        }
    }
}

impl<T> FixedArray<T> {
    pub fn new<A: Allocator>(len: usize, allocator: &mut A) -> Result<FixedArray<T>, Error> {
        match allocator.alloc(len * size_of::<T>(), align_of::<T>()) {
            Ok(ptr) => Ok(FixedArray {
                ptr: ptr as *mut T,
                len: len,
            }),
            Err(err) => Err(err),
        }
    }
}

pub struct VirtualArena {
    base: *const u8,
    reserved: usize,
    committed: usize,
    used: usize,
}

impl Default for VirtualArena {
    fn default() -> VirtualArena {
        VirtualArena {
            base: core::ptr::null_mut(),
            reserved: 0,
            committed: 0,
            used: 0,
        }
    }
}

impl VirtualArena {
    pub fn new(reserve: usize, commit: usize) -> VirtualArena {
        assert!(commit <= reserve);
        let (base, reserved) = crate::platform_mem::reserve(reserve);
        let committed = crate::platform_mem::commit(base, commit);
        let used = 0;
        VirtualArena{base, reserved, committed, used}
    }
}

impl Allocator for VirtualArena {
    fn alloc(self: &mut Self, size: usize, align: usize) -> Result<*const u8, Error> {
        let (base_aligned, size_aligned) =
            align_ptr((self.base as usize + self.used) as *const u8, align, size);

        let committed_and_free = self.committed - self.used;
        let reserved_and_free = self.reserved - self.used;

        if committed_and_free >= size_aligned {
            self.used += size_aligned;
            Ok(base_aligned)
        } else if reserved_and_free >= size_aligned {
            self.committed = crate::platform_mem::commit(self.base, self.used + size_aligned);
            self.used += size_aligned;
            Ok(base_aligned)
        } else {
            Err(Error::OutOfMemory)
        }
    }
}
