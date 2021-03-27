#![feature(aarch64_target_feature)]
#![feature(stdsimd)]
#![feature(test)]
use std::arch;
use std::arch::aarch64::{__crc32b, __crc32d};
use unroll::unroll_for_loops;
extern crate test;

trait Crc32T {
	fn crc32(d: &[u8]) -> u32;
}

pub struct Crc32Slow {}
pub struct Crc32Fast {}

impl Crc32T for Crc32Slow {
	fn crc32(d: &[u8]) -> u32 {
		let mut sum = 0xFFFFFFFF;

		for index in 0..d.len() {
			let mut cur = d[index] as u8;
			for _sub in 0..8 {
				let quad = (cur as u32 ^ sum) & 1;
				sum >>= 1;
				if quad > 0 {
					sum ^= 0xEDB88320;
				}
				cur >>= 1;
			}
		}

		!sum
	}
}

#[target_feature(enable = "crc")]
#[unroll_for_loops]
unsafe fn aarch64_crc32(d: &[u8]) -> u32 {
	let mut sum = 0xFFFFFFFF;
	let (block_a, data, block_b) = d.align_to::<u64>();

	sum = block_a.iter().fold(sum, |acc, &b| std::arch::aarch64::__crc32b(acc, b));
	let mut quad = data.chunks_exact(8);
	for index in &mut quad {
		for qindex in 0..8 {
			sum = std::arch::aarch64::__crc32d(sum, index[qindex]);
		}
	}
	sum = quad.remainder().iter().fold(sum, |acc, &b| std::arch::aarch64::__crc32d(acc, b));
	sum = block_b.iter().fold(sum, |acc, &b| std::arch::aarch64::__crc32b(acc, b));
	!sum
}

impl Crc32T for Crc32Fast {

	fn crc32(d: &[u8]) -> u32 {
		let mut sum = 0;
		unsafe { sum = aarch64_crc32(d) }
		sum
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crc32_impl_cmp() {
	let data1cksum = Crc32Slow::crc32("Crc32Slow".as_bytes());
	let data2cksum = Crc32Slow::crc32(&(3.14_f32).to_be_bytes());
	assert!(data1cksum == 3091428579);
	assert!(data2cksum == 2944392619);
	let data1cksumfast = Crc32Fast::crc32("Crc32Slow".as_bytes());
	let data2cksumfast = Crc32Fast::crc32(&(3.14_f32).to_be_bytes());
	assert!(data1cksum == data1cksumfast);
	assert!(data2cksum == data2cksumfast);
    }

    #[bench]
    fn crc32_bench_slow(b: &mut test::Bencher) {
	let smp : u64 = 0xDEADBEEF as u64;

        b.iter(|| 
            Crc32Slow::crc32(&smp.to_be_bytes())
        );
    }

    #[bench]
    fn crc32_bench_fast(b: &mut test::Bencher) {
	let smp : u64 = 0xDEADBEEF as u64;

        b.iter(|| 
            Crc32Fast::crc32(&smp.to_be_bytes())
        );
    }
}
