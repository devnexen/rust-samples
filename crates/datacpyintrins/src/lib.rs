#![feature(test)]
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_load_si128, _mm_store_si128, __m128i};
use std::mem::size_of;
extern crate test;
const LEN : usize = size_of::<__m128i>();

pub fn datacpy(dest: &mut [u8], src: &[u8], sz: usize) {
    let mut i: usize = 0;
    unsafe {
        let srcptr = src.as_ptr();
        let mut destptr = dest.as_mut_ptr();
        let boundaries = src[src.len()..].as_ptr();
        let mut s = srcptr;
        let mut d = destptr;
        while s <= boundaries.sub(LEN) {
            let sm128 = _mm_load_si128(s as *const __m128i);
            _mm_store_si128(d as *mut __m128i, sm128);
            s = s.add(LEN);
            d = d.add(LEN);
        }
    }
}

pub fn datacpybg(dest: &mut [u8], src: &[u8], sz: usize) {
    let mut i: usize = 0;
    unsafe {
        let srcptr = src.as_ptr();
        let mut destptr = dest.as_mut_ptr();
        let boundaries = src[src.len()..].as_ptr();
        let mut s = srcptr;
        let mut d = destptr;
        while s <= boundaries.sub(LEN) {
            let sm1280 = _mm_load_si128(s as *const __m128i);
            let sm1281 = _mm_load_si128(s.add(LEN) as *const __m128i);
            let sm1282 = _mm_load_si128(s.add(2 * LEN) as *const __m128i);
            let sm1283 = _mm_load_si128(s.add(3 * LEN) as *const __m128i);
            _mm_store_si128(d as *mut __m128i, sm1280);
            _mm_store_si128(d.add(LEN) as *mut __m128i, sm1281);
            _mm_store_si128(d.add(2 * LEN) as *mut __m128i, sm1282);
            _mm_store_si128(d.add(3 * LEN) as *mut __m128i, sm1283);
            s = s.add(4 * LEN);
            d = d.add(4 * LEN);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut src : Vec<u8> = Vec::new();
        for i in 0..16 {
            src.push(i);
        }
        let mut dest: Vec<u8> = Vec::new();
        for i in 0..16 {
            dest.push(0);
        }
        datacpy(&mut dest, &src, src.len());
        assert_eq!(dest, src);
        src = Vec::with_capacity(128);
        for i in 0..128 {
            src.push(i);
        }
        dest = Vec::with_capacity(128);
        for i in 0..128 {
            dest.push(0);
        }
        datacpybg(&mut dest, &src, src.len());
        assert_eq!(dest, src);
    }

    #[bench]
    fn cpybgslow(b: &mut test::Bencher) {
        let mut src = Vec::with_capacity(128);
        for i in 0..128 {
            src.push(i);
        }
        let mut dest = Vec::with_capacity(128);
        for i in 0..128 {
            dest.push(0);
        }
        b.iter(|| 
            datacpy(&mut dest, &src, src.len())
        );
    }

    #[bench]
    fn cpybg(b: &mut test::Bencher) {
        let mut src = Vec::with_capacity(128);
        for i in 0..128 {
            src.push(i);
        }
        let mut dest = Vec::with_capacity(128);
        for i in 0..128 {
            dest.push(0);
        }
        b.iter(|| 
            datacpybg(&mut dest, &src, src.len())
        );
    }
}
