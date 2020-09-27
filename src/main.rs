use spiral::*;
use std::collections::HashMap;

fn main() {
	let spiral = ChebyshevIterator::new(0, 0, 64);

	let mut occ: [u32; 16] = [0; 16];

	for (x, z) in spiral {
		for y in 63..128 {
			let o = grass_offset(x, y, z);

			occ[o.x as usize] += 1;
			occ[o.y as usize] += 1;
			occ[o.z as usize] += 1;
		}
	}

	for (i, c) in occ.iter().enumerate() {
		println!("{:>3} {:>12}", i, c);
	}
}

const X_MULT: i32 = 0x2fc20f;
const Y_MULT: i32 = 0x6ebfff5;
const LCG_MULT: i64 = 0x285b825;
const LCG_ADDEND: i64 = 11;

fn grass_offset(x: i32, y: i32, z: i32) -> Offset {
	let mut seed = (x * X_MULT) as i64 ^ (y * Y_MULT) as i64 ^ z as i64;
	seed = seed * seed * LCG_MULT + seed * LCG_ADDEND;
	Offset {
		x: (seed >> 16 & 15) as u8,
		y: (seed >> 20 & 15) as u8,
		z: (seed >> 24 & 15) as u8,
	}
}

#[derive(Copy, Clone, Debug)]
struct Position {
	x: i32,
	y: i32,
	z: i32,
}

#[derive(Copy, Clone, Debug)]
struct Offset {
	x: u8,
	y: u8,
	z: u8,
}
