use crate::assert;

pub const BYTE: usize = 1;
pub const KILOBYTE: usize = 1024 * BYTE;
pub const MEGABYTE: usize = 1024 * KILOBYTE;
pub const GIGABYTE: usize = 1024 * MEGABYTE;

fn is_power_of_2(val: usize) -> bool {
    let result = (val & (val - 1)) == 0;
    return result;
}

fn align_ptr(ptr: &mut *const u8, align: usize, size: &mut usize) {
    assert(is_power_of_2(align));
    let ptr_og = *ptr;
    let off_by = ptr_og as usize & (align - 1);
    let mut move_by = 0;
    if off_by > 0 {
        move_by = align - off_by;
    }
    let ptr_aligned = ptr_og as usize + move_by;
    *ptr = ptr_aligned as *const u8;
    *size = *size + move_by;
}

pub enum Error {
    OutOfMemory,
}

pub trait Allocator {
    fn alloc(self: &mut Self, size: usize, align: usize) -> Result<*const u8, Error>;
    fn make<T>(self: &mut Self, len: usize) -> Result<&mut [T], Error>;
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
        assert(commit <= reserve);
        self.reserved = reserve;
        self.used = 0;
        crate::platform_mem::reserve(reserve, &mut self.base, &mut self.reserved);
        crate::platform_mem::commit(self.base, commit, &mut self.committed);
    }
}

impl Allocator for VirtualArena {
    fn alloc(self: &mut Self, size: usize, align: usize) -> Result<*const u8, Error> {
        let result;

        let mut base_aligned = (self.base as usize + self.used) as *const u8;
        let mut size_aligned = size;
        align_ptr(&mut base_aligned, align, &mut size_aligned);

        let committed_and_free = self.committed - self.used;
        let reserved_and_free = self.reserved - self.used;

        if committed_and_free >= size_aligned {
            result = Ok(base_aligned);
            self.used += size_aligned;
        } else if reserved_and_free >= size_aligned {
            crate::platform_mem::commit(self.base, self.used + size_aligned, &mut self.committed);
            result = Ok(base_aligned);
            self.used += size_aligned;
        } else {
            result = Err(Error::OutOfMemory);
        }

        result
    }

    fn make<T>(self: &mut Self, len: usize) -> Result<&mut [T], Error> {
        let result;
        match self.alloc(core::mem::size_of::<T>() * len, core::mem::size_of::<T>()) {
        	Ok(ptr) => {
        		let slice = core::ptr::slice_from_raw_parts_mut(ptr as *mut T, len);
        		result = Ok(unsafe { &mut *slice })
        	}
        	Err(err) => {result = Err(err)}
        }
        result
    }
}
