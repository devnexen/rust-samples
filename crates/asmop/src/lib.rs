#![feature(asm)]
#![feature(test)]
extern crate libc;
extern crate test;
use libc::c_void;

extern "C" {
    pub fn arc4random_buf(a: *mut c_void, b: usize) -> c_void;
}

pub fn add64(x: u32, y: u32) -> u64 {
    let mut r: u64 = 0;
    unsafe {
        asm!("mov x2, {0:x}",
             "mov x3, {1:x}",
             "add {2:x}, x2, x3",
             in(reg) x,
             in(reg) y,
             out(reg) r,
             );
    }
    r
}

pub fn add128(x: u64, y: u64) -> u128 {
    let mut r: u128 = 0;
    let hx = x >> 32;
    let lx = x & 0xFFFFFFFF;
    let hy = y >> 32;
    let ly = y & 0xFFFFFFFF;
    let mut hr: u64 = 0;
    let mut lr: u64 = 0;
    unsafe {
        asm!("mov x2, {0:x}",
             "mov x3, {1:x}",
             "mov x4, {2:x}",
             "mov x5, {3:x}",
             "adds {4:x}, x3, x5",
             "adc {5:x}, x2, x4",
             in(reg) lx,
             in(reg) hx,
             in(reg) ly,
             in(reg) hy,
             out(reg) lr,
             out(reg) hr
             );
        r = (lr + hr) as u128;
    }
    r
}

pub fn mul64(x: u32, y: u32) -> u64 {
    let mut r: u64 = 0;
    unsafe {
        asm!("umull {2:x}, {0:w}, {1:w}",
             in(reg) x,
             in(reg) y,
             out(reg) r
             );
    }
    r
}

pub fn mul128(x: u64, y: u64) -> u128 {
    let mut r: u128 = 0;
    let mut lr: u64 = 0;
    let mut hr: u64 = 0;
    unsafe {
        asm!("mul {2:x}, {0:x}, {1:x}",
             "umulh {3:x}, {0:x}, {1:x}",
             in(reg) x,
             in(reg) y,
             out(reg) lr,
             out(reg) hr
             );
        r = (lr + hr) as u128;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn adds() {
        let mut x = 1u32;
        let mut y = 1u32;
        let mut r = add64(x, y);
        assert_eq!(r, 2u64);
        x = !0u32;
        y = !x;
        r = add64(x, y);
        assert_eq!(r, 0xFFFFFFFF);
        let mut a = 1u64;
        let mut b = 1u64;
        let mut c = add128(a, b);
        assert_eq!(c, 2u128);
        a = !0u64;
        b = !a;
        c = add128(a, b);
        assert_eq!(c, 0x1FFFFFFFE);
        a = 10u64;
        b = a * 2u64;
        c = add128(a, b);
        assert_eq!(c, 30u128);
    }

    #[test]
    fn muls() {
        let mut x = 1u32;
        let mut y = 1u32;
        let mut r = mul64(x, y);
        assert_eq!(x as u64, r);
        let mut a = 2u32;
        let mut b = 2u32;
        let mut c = mul64(a, b);
        assert_eq!(c, 2u32.pow(2u32) as u64);
        let mut a = 1u64;
        let mut b = 1u64;
        let mut c = mul128(a, b);
        assert_eq!(c, 1 as u128);
        a = 10u64;
        b = 30u64;
        c = mul128(a, b);
        assert_eq!(c, 300u128);
        a = 300u64;
        b = 25u64;
        c = mul128(a, b);
        assert_eq!(c, 7500u128);
    }

    #[bench]
    fn adds64(b: &mut test::Bencher) {
        let mut x : u32 = 0;
        let mut y : u32 = 0;
        let mut ptrx : *mut u32 = &mut x;
        let mut ptry : *mut u32 = &mut y;
        const sz : usize = size_of::<u32>();
        unsafe {
            arc4random_buf(ptrx as *mut c_void, sz);
            arc4random_buf(ptry as *mut c_void, sz);
        }
        b.iter(|| 
            add64(x, y)
        );
    }

    #[bench]
    fn adds128(b: &mut test::Bencher) {
        let mut x : u64 = 0;
        let mut y : u64 = 0;
        let mut ptrx : *mut u64 = &mut x;
        let mut ptry : *mut u64 = &mut y;
        const sz : usize = size_of::<u64>();
        unsafe {
            arc4random_buf(ptrx as *mut c_void, sz);
            arc4random_buf(ptry as *mut c_void, sz);
        }
        b.iter(|| 
            add128(x, y)
        );
    }
}
