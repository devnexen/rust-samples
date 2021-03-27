trait Crc32T {
	fn crc32(d: &[u8]) -> u32;
}

pub struct Crc32Slow {}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crc32_impl_cmp() {
	let data1cksum = Crc32Slow::crc32("Crc32Slow".as_bytes());
	let data2cksum = Crc32Slow::crc32(&(3.14_f32).to_be_bytes());
	assert!(data1cksum == 3091428579);
	assert!(data2cksum == 2944392619);
    }
}
