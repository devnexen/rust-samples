#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_loadu_si128, _mm_storeu_si128, __m128i};
use std::mem::size_of;
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
            let sm128 = _mm_loadu_si128(s as *const __m128i);
            _mm_storeu_si128(d as *mut __m128i, sm128);
            s = s.add(LEN);
            d = d.add(LEN);
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
    }
}
