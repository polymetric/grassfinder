use spiral::*;
use std::fs;
use std::error::Error;
use std::ops::*;
use std::env;

const PATH: &'static str = "offsets.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

	let spiral = ChebyshevIterator::new(0, 0, 2048);
	let rows = load_grass_positions().unwrap();

    let recorigin = Position::new(
        (&args[1]).parse::<i32>().unwrap(),
        (&args[2]).parse::<i32>().unwrap(),
        (&args[3]).parse::<i32>().unwrap(),
    );

    let grass_count = rows.len();

	for (x, z) in spiral {
		for y in 62..80 {
			let mut delta: f64 = 0.0;
			let testpos = Position { x, y, z };

			for (pos, off) in rows.iter() {
				let pos_abs = testpos + *pos - recorigin;
				let temp = grass_offset_from_pos(pos_abs);
				let mut is_match = "";

//				if (*off - grass_offset_from_pos(pos_abs)).abs() < 2 {
//					matches += 1;
//					is_match = "MATCH";
//				}
                delta += (*off - grass_offset_from_pos(pos_abs)).abs() as f64;

//				println!();
//				println!("{:>8}{:>8}{:>8}", pos_abs.x, pos_abs.y, pos_abs.z);
//				println!("{:>8}{:>8}{:>8}{:>8}{:>8}{:>8} {}", off.x, off.y, off.z, temp.x, temp.y, temp.z, is_match);
			}

            delta /= grass_count as f64;

			if delta < 4.0 {
				println!(
					"{:>8}{:>8}{:>8} has delta of {:.3}",
					x, y, z, delta
				);
			}
		}
	}
}

const X_MULT: i32 = 0x2fc20f;
const Z_MULT: i32 = 0x6ebfff5;
const LCG_MULT: i64 = 0x285b825;
const LCG_ADDEND: i64 = 11;

fn grass_offset_from_pos(p: Position) -> Offset {
	grass_offset(p.x, p.y, p.z)
}

fn grass_offset(x: i32, y: i32, z: i32) -> Offset {
	let mut seed = (x * X_MULT) as i64 ^ (z * Z_MULT) as i64 ^ y as i64;
	seed = seed * seed * LCG_MULT + seed * LCG_ADDEND;
	Offset {
		x: (seed >> 16 & 15) as i8,
		y: (seed >> 20 & 15) as i8,
		z: (seed >> 24 & 15) as i8,
	}
}

// returns a tuple of the input integer offsets converted to
// actual position offsets (1.0 is equal to 1 block)
fn off_itof_xyz(x: i8, y: i8, z: i8) -> (f64, f64, f64) {
	(
		map(x as f64, 0.0, 15.0, -0.25, 0.25),
		map(y as f64, 0.0, 15.0, -0.20, 0.00),
		map(z as f64, 0.0, 15.0, -0.25, 0.25),
	)
}

fn off_ftoi_xyz(x: f64, y: f64, z: f64) -> (i8, i8, i8) {
	(
		map(x, -0.25, 0.25, 0.0, 15.0) as i8,
		map(y, -0.2, 0.0, 0.0, 15.0) as i8,
		map(z, -0.25, 0.25, 0.0, 15.0) as i8,
	)
}

// standard linear interp
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

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Copy, Clone, Debug)]
struct Offset {
	x: i8,
	y: i8,
	z: i8,
}

impl Offset {
    fn abs(&self) -> i8 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Sub for Offset {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialEq for Offset {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y &&
		self.z == other.z
	}
}

impl Sub for Position {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl Add for Position {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

fn load_grass_positions() -> Result<Vec<(Position, Offset)>, Box<dyn Error>> {
	let contents = fs::read_to_string(PATH)?;

	Ok(contents
		.lines()
		.map(|line| {
			let mut line = line
				.split_whitespace();

			(
				Position {
					x: line.next().unwrap().parse::<i32>().unwrap(),
					y: line.next().unwrap().parse::<i32>().unwrap(),
					z: line.next().unwrap().parse::<i32>().unwrap(),
				},
				Offset {
					x: line.next().unwrap().parse::<i8>().unwrap(),
					y: line.next().unwrap().parse::<i8>().unwrap(),
					z: line.next().unwrap().parse::<i8>().unwrap(),
				}
			)
		})
		.collect::<Vec<(Position, Offset)>>())
}
