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
    fn make<T>(self: &mut Self, len: usize) -> Result<*mut [T], Error>;
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
    pub fn init(self: &mut VirtualArena, reserve: usize, commit: usize) {
        assert!(commit <= reserve);
        self.reserved = reserve;
        self.used = 0;
        (self.base, self.reserved) = crate::platform_mem::reserve(reserve);
        self.committed = crate::platform_mem::commit(self.base, commit);
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

    fn make<T>(self: &mut Self, len: usize) -> Result<*mut [T], Error> {
        match self.alloc(core::mem::size_of::<T>() * len, core::mem::align_of::<T>()) {
            Ok(ptr) => {
                let slice = core::ptr::slice_from_raw_parts_mut(ptr as *mut T, len);
                Ok(slice as *mut [T])
            }
            Err(err) => Err(err),
        }
    }
}
