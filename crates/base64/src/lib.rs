use std::str;

pub struct Base64 {}

const ENCTABLE : [char; 64] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
	'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
	'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
	'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
	'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
	'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
	'w', 'x', 'y', 'z', '0', '1', '2', '3',
	'4', '5', '6', '7', '8', '9', '+', '/'
];

const DECTABLE : [i32; 256] = [
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -1, -2, -2, -1, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 62, -2, -2, -2, 63,
	52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -2, -2, -2, -2, -2, -2,
	-2,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
	15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -2, -2, -2, -2, -2,
	-2, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
	41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
	-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2
];

const MODS : [i32; 3] = [0, 2, 1];

impl Base64 {
	pub fn encode(data: &[u8]) -> Vec<u8> {
		let size = 4 * (data.len() + 2) / 3;
		let mut ret : Vec<u8> = vec![0; size];
		let mut i : usize = 0;
		let mut j : usize = 0;
		while i < data.len() {
			let mut a = 0 as u32;
			let mut b = 0 as u32;
			let mut c = 0 as u32;
			if i < data.len() {
				a = data[i] as u32;
			} else {
				a = 0;
			}
			i += 1;
			if i < data.len() {
				b = data[i] as u32;
			} else {
				b = 0;
			}
			i += 1;
			if i < data.len() {
				c = data[i] as u32;
			} else {
				c = 0;
			}
			i += 1;
			let d = (a<<0x10) + (b<<0x08) + c;
			ret[j] = ENCTABLE[(d>>3*6) as usize & 0x3f] as u8;
			j += 1;
			ret[j] = ENCTABLE[(d>>2*6) as usize & 0x3f] as u8;
			j += 1;
			ret[j] = ENCTABLE[(d>>1*6) as usize & 0x3f] as u8;
			j += 1;
			ret[j] = ENCTABLE[(d>>0*6) as usize & 0x3f] as u8;
			j += 1;
		}
		i = 0;
		while i < MODS[data.len() as usize % 3] as usize {
			ret[size - 1 - i] = '=' as u8;
			i += 1;
		}
		ret
	}

	pub fn decode(input: &[u8]) -> Vec<u8> {
		assert!(input.len() % 4 == 0);
		let mut size = (input.len() / 4) * 3;
		let mut i : usize = 0;
		let mut j : usize = 0;
		if input[input.len() - 1] == '=' as u8 {
			size = size - 1;
		}
		if input[input.len() - 2] == '=' as u8 {
			size = size - 1;
		}
		let mut ret : Vec<u8> = vec![0; size];
		while i < input.len() {
			let mut a = 0 as i32;
			let mut b = 0 as i32;
			let mut c = 0 as i32;
			let mut d = 0 as i32;
			if input[i] == '=' as u8 {
				a = 0 & i as i32;
			} else {
				a = DECTABLE[input[i] as usize];
			}
			i += 1;
			if input[i] == '=' as u8 {
				b = 0 & i as i32;
			} else {
				b = DECTABLE[input[i] as usize];
			}
			i += 1;
			if input[i] == '=' as u8 {
				c = 0 & i as i32;
			} else {
				c = DECTABLE[input[i] as usize];
			}
			i += 1;
			if input[i] == '=' as u8 {
				d = 0 & i as i32;
			} else {
				d = DECTABLE[input[i] as usize];
			}
			i += 1;
			let e = (a<<3*6) as i32 + (b<<2*6) as i32 + (c<<1*6) as i32 + (d<<0*6) as i32;
			if j < size {
				ret[j] = (e>>2*8) as u8 & 0xFF as u8;
				j += 1;
			}
			if j < size {
				ret[j] = (e>>1*8) as u8 & 0xFF as u8;
				j += 1;
			}
			if j < size {
				ret[j] = (e>>0*8) as u8 & 0xFF as u8;
				j += 1;
			}
		}
		ret
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64impl() {
	let encoded = Base64::encode("TEST".as_bytes());
	let encodedstr = str::from_utf8(&encoded).unwrap();
	assert_eq!("VEVTVA==", encodedstr);
	let decoded = Base64::decode(encodedstr.as_bytes());
	let decodedstr = str::from_utf8(&decoded).unwrap();
	println!("{}", decodedstr);
	assert_eq!("TEST", decodedstr);
    }
}
