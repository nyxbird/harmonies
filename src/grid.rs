use std::prelude::*;
use crate::util::{*, Color::*};
use crate::card::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hex {
	// Blank is a non-space for grid shaping
	Blank, #[default] Empty,
	Wood, TwoWood,
	Leaf, Tree, TwoTree,
	Rock, TwoRock, ThreeRock,
	Brick, Building,
	Field,
	Water
}

use Hex::*;

impl Hex {
	// stack another hex on this one
	pub fn stack(&self, top: &Hex) -> Option<Hex> {
		match (*self, *top) {
			(Empty, x) => Some(x),
			(Wood, Wood) => Some(TwoWood),
			(Wood, Leaf) => Some(Tree),
			(TwoWood, Leaf) => Some(TwoTree),
			(Rock, Rock) => Some(TwoRock),
			(TwoRock, Rock) => Some(ThreeRock),
			(Wood|Rock|Brick, Brick) => Some(Building),
			_ => None
		}
	}

	pub fn color(&self) -> Color {
		use crate::util::Color::*;
		match self {
			Blank|Empty => Norm,
			Wood|TwoWood => Magenta,
			Leaf|Tree|TwoTree => Green,
			Rock|TwoRock|ThreeRock => Cyan,
			Brick|Building => Red,
			Field => Yellow,
			Water => Blue
		}
	}

	pub fn char(&self, lined: bool) -> char {
		match self {
			Wood|TwoWood => 't',
			Leaf|Tree|TwoTree => 'l',
			Rock|TwoRock|ThreeRock => 'r',
			Brick|Building => 'b',
			Field => 'f',
			Water => 'w',
			Blank|Empty => ' '
		}
	}

	pub fn show(&self, lined: bool) -> String {
		self.color().ansi() +
		&(if lined {color_under()} else {String::new()}) +
		(match self {
			Blank => "  ",
			Empty => "╶╴",
			Wood => "-t",
			TwoWood => "=t",
			Leaf => "-l",
			Tree => "=l",
			TwoTree => "≡l",
			Rock => "-r",
			TwoRock => "=r",
			ThreeRock => "≡r",
			Brick => "-b",
			Building => "=b",
			Field => "-f",
			Water => "-w"
		}) + &color_reset()
	}
}

pub struct Grid {
	pub h: usize, pub w: usize,
	pub hexes: Vec<Vec<Hex>>,
	pub cubes: Vec<Vec<bool>>
}

impl Grid {
	//pub fn new(h: usize, w: usize) -> Self {
	//	let mut hexes = Vec::with_capacity(h);
	//	let mut cubes = Vec::with_capacity(h);
	//	for i in 0..h {
	//		let l = w as i32-(i as i32-h as i32/2).abs();
	//		hexes.push(vec![Hex::Empty; l as usize]);
	//		cubes.push(vec![false; l as usize]);
	//	}
	//	Grid{h, w, hexes, cubes}
	//}
	//
	// standard grids are 5x5 and 7x4
	pub fn new_rect(h: usize, w: usize) -> Self {
		let mut hexes = Vec::with_capacity(h);
		let mut cubes = Vec::with_capacity(h);
		for i in 0..h {
			let l = w-if i%2==1 {1} else {0};
			hexes.push(vec![Hex::Empty; l as usize]);
			cubes.push(vec![false; l as usize]);
		}
		Grid {h, w, hexes, cubes}
	}

	pub fn stack(&mut self, y: usize, x: usize, h: &Hex) -> Option<()> {
		*self.hexes.get_mut(y)?.get_mut(x)? =
			self.hexes.get(y)?.get(x)?.stack(h)?;
		Some(())
	}

	pub fn get(&self, y: usize, x: usize) -> Option<Hex> {
		Some(*self.hexes.get(y)?.get(x)?)
	}

	// get, but with doubled x coords
	pub fn get2x(&self, y: isize, x: isize) -> Option<Hex> {
		if y < 0 || x < 0 {return None;}
		self.get(y as usize, x as usize/2)
	}

	pub fn get_cube2x(&self, y: isize, x: isize) -> bool {
		if y < 0 || x < 0 {return false;}
		self.cubes[y as usize][x as usize/2]
	}

	// [doubled x] rotate pi/3 clockwise about (0, 0)
	pub fn rot(y: isize, x: isize) -> (isize, isize) {
		((y+x)/2, (-3*y+x)/2)
	}

	// yeah i have no idea either
	pub fn valid_cube(&mut self, y: isize, x: isize, c: &Card) -> bool {
		// NOTE: x values are often doubled
		let offs = vec![(-1, -1), (-1, 1), (0, -2), (0, 0), (0, 2)];
		let mut map = Vec::new();
		for i in 0..5 {
			map.push((offs[i], c.reqs[i]));
		}
		for (p, _) in map.iter_mut() {
			p.0 -= offs[c.loc].0 as isize;
			p.1 -= offs[c.loc].1 as isize;
		}
		// try each possible rotation and cell
		for _ in 0..6 {
			let dx2 = x*2+(self.h as isize%2);
			let c = self.get2x(y, dx2);
			if c == None {continue;}
			if self.get_cube2x(y, dx2) {continue;}
			let mut yum = true;
			for &(p, h) in map.iter() {
				if h != Hex::Empty && self.get2x(y+p.0, dx2+p.1) != Some(h) {
					yum = false; break;
				}
			}
			if yum {return true}
			// rotate
			for (p, _) in map.iter_mut() {
				*p = Self::rot(p.0, p.1);
			}
		}
		return false;
	}
}

//   a b c d e
// 0 . . . . .
// 1  . . . .
// 2 . . . . .
// 3  . . . .
// 4 . . . . .
//
//   a b c d
// 0 . . . .
// 1  . . .
// 2 . . . .
// 3  . . .
// 4 . . . .
// 5  . . .
// 6 . . . .
