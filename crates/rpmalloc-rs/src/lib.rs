#![no_std]

use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use libc::size_t;

extern "C" {
    pub fn rpaligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn rpaligned_realloc(ptr: *mut c_void, alignment: size_t, size: size_t) -> *mut c_void;
    pub fn rpfree(ptr: *mut c_void) -> c_void;
}

pub struct Rpmalloc;

unsafe impl GlobalAlloc for Rpmalloc {
    #[inline(always)]
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if l.align() % 2 != 0 {
            return core::ptr::null_mut();
        }
        rpaligned_alloc(l.align(), l.size()) as *mut u8
    }
    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, l: Layout, ns: usize) -> *mut u8 {
        if l.align() % 2 != 0 {
            return core::ptr::null_mut();
        }
        rpaligned_realloc(ptr as *mut c_void, l.align(), ns as size_t) as *mut u8
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _l: Layout) {
        rpfree(ptr as *mut c_void);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn Rpmalloc_panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn straightforward() {
        unsafe {
            let l = Layout::from_size_align_unchecked(64, 16);
            let nm = Rpmalloc;
            let ptr = nm.alloc(l);
            nm.dealloc(ptr, l);
        }
    }
}
