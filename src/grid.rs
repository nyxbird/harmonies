use std::prelude::*;
use crate::util::{*, Color::*};
use crate::card::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
	#[default] Empty,
	Wood, Leaf,
	Rock, Brick,
	Field, Water
}

impl Tile {
	pub fn color(&self) -> Color {
		use crate::util::Color::*;
		match self {
			Empty => Norm,
			Wood => Magenta, Leaf => Green,
			Rock => Cyan, Brick => Red,
			Field => Yellow, Water => Blue
		}
	}

	pub fn symbol(&self) -> char {
		match self {
			Empty => '-',
			Wood => 't', Leaf => 'l',
			Rock => 'r', Brick => 'b',
			Field => 'f', Water => 'w'
		}
	}

	pub fn cube(&self) -> char {
		match self {
			Empty => '-',
			Wood => 'T', Leaf => 'L',
			Rock => 'R', Brick => 'B',
			Field => 'F', Water => 'W'
		}
	}

	pub fn show(&self) -> String {
		format!("{}{}{}", self.color().ansi(), self.symbol(), Color::Norm.ansi())
	}
}

use Tile::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hex {
	pub t: Tile,
	pub h: u8,
	pub cube: bool
}

impl Hex {
	pub fn stack(&self, t: &Tile) -> Option<Hex> {
		match (*self, *t) {
			(Hex{cube:true,..}, _) => None,
			(Hex{t:Empty,..}, t) => Some(Hex::new(t, 1)),
			(Hex{t:Wood, h:1,..}, Wood) => Some(Hex::new(Wood, 2)),
			(Hex{t:Wood, h:x@(1|2),..}, Leaf) => Some(Hex::new(Leaf, x+1)),
			(Hex{t:Rock, h:x@(1|2),..}, Rock) => Some(Hex::new(Rock, x+1)),
			(Hex{t:(Wood|Rock|Brick), h:1,..}, Brick) => Some(Hex::new(Brick, 2)),
			_ => None
		}
	}

	pub fn new(t: Tile, h: u8) -> Hex {
		Hex{t, h, cube: false}
	}

	pub fn cube(t: Tile, h: u8) -> Hex {
		Hex{t, h, cube: true}
	}

	pub fn level(&self) -> char {
		match self.h {
			1 => '-', 2 => '=',
			3 => '≡', _ => ' '
		}
	}
	
	pub fn show(&self) -> String {
		if self.t == Empty {return "╶╴".to_string()}
		format!("{}{}{}{}",
			self.t.color().ansi(),
			self.level(),
			if self.cube {self.t.cube()} else {self.t.symbol()},
			Color::Norm.ansi()
		)
	}

	pub fn empty() -> Self {
		Hex{t: Empty, h: 0, cube: false}
	}
}

pub struct Grid {
	pub h: usize, pub w: usize,
	pub hexes: Vec<Vec<Hex>>,
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
		for i in 0..h {
			let l = w-if i%2==1 {1} else {0};
			hexes.push(vec![Hex::empty(); l as usize]);
		}
		Grid {h, w, hexes}
	}

	pub fn stack(&mut self, y: usize, x: usize, h: &Tile) -> Option<()> {
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

	// [doubled x] rotate pi/3 clockwise about (0, 0)
	pub fn rot(y: isize, x: isize) -> (isize, isize) {
		((y+x)/2, (-3*y+x)/2)
	}

	pub fn valid_cube(&mut self, y: isize, x: isize, c: &Card) -> bool {
		// NOTE: x values are often doubled
		let offs = vec![(-1, -1), (-1, 1), (0, -2), (0, 0), (0, 2)];
		let mut map = Vec::new();
		for i in 0..5 {
			if c.reqs[i].t != Empty {map.push((offs[i], c.reqs[i]))}
		}
		for (p, _) in map.iter_mut() {
			p.0 -= offs[c.loc].0 as isize;
			p.1 -= offs[c.loc].1 as isize;
		}
		// try each possible rotation and cell
		for r in 0..6 {
			let dx2 = x*2+(y as isize%2);
			let Some(Hex{cube: false,..}) = self.get2x(y, dx2) else {continue};
			let mut yum = true;
			for &(p, h) in map.iter() {
				if self.get2x(y+p.0, dx2+p.1) != Some(Hex{cube: false,..h}) {
					// TODO: fix broken card matching
					print!("![{}{}{}]{}/{} ", r, y+p.0, dx2+p.1, 
						if let Some(h0) = self.get2x(y+p.0, dx2+p.1) {h0.show()} else {"??".to_string()},		 
					h.show());
					yum = false;// break;
				}
				else {
					print!("({}{}{}){}/{} ", r, y+p.0, dx2+p.1, 
						if let Some(h0) = self.get2x(y+p.0, dx2+p.1) {h0.show()} else {"??".to_string()},		 
					h.show());
				}
				//if let Some(h0) = self.get2x(y+p.0, dx2+p.1) && h0.t == h.t && h0.h == h.h {}
				//else {
				//	yum = false; break;
				//}
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
