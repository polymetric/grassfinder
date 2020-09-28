use spiral::*;
use std::collections::HashMap;
use std::fs;
use std::error::Error;

const PATH: &'static str = "positions.txt";

fn main() {
	let spiral = ChebyshevIterator::new(0, 0, 2048);
	let rows = load_grass_positions().unwrap();

	for row in rows {
		println!("{:?}", row);
	}

	for (i, (x, z)) in spiral.enumerate() {
		for y in 63..128 {
			let oi = grass_offset(x, y, z);
			let of: (f64, f64, f64) = off_itof_xyz(oi.x, oi.y, oi.z);

//			println!("{:>6} {:>6} {:>6} {:>12} {:>12} {:>12}", x, y, z, oi.x, oi.y, oi.z);
//			println!("{:>6} {:>6} {:>6} {:>12.6} {:>12.6} {:>12.6}", x, y, z, of.0, of.1, of.2);
		}
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

fn off_itof_xyz(x: u8, y: u8, z: u8) -> (f64, f64, f64) {
	(
		map(x as f64, 0.0, 15.0, -0.25, 0.25),
		map(y as f64, 0.0, 15.0, -0.20, 0.00),
		map(z as f64, 0.0, 15.0, -0.25, 0.25),
	)
}

fn map(
	x: f64,
	in_min: f64,
	in_max: f64,
	out_min: f64,
	out_max: f64
) -> f64 {
	(x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
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

fn load_grass_positions() -> Result<Vec<(Position, Offset)>, Box<dyn Error>> {
	let contents = fs::read_to_string(PATH)?;

	Ok(contents
		.lines()
		.map(|line| {
			let mut line = line.split(" ");

			(
				Position {
					x: line.next().unwrap().parse::<i32>().unwrap(),
					y: line.next().unwrap().parse::<i32>().unwrap(),
					z: line.next().unwrap().parse::<i32>().unwrap(),
				},
				Offset {
					x: line.next().unwrap().parse::<u8>().unwrap(),
					y: line.next().unwrap().parse::<u8>().unwrap(),
					z: line.next().unwrap().parse::<u8>().unwrap(),
				}
			)
		})
		.collect::<Vec<(Position, Offset)>>())
}
