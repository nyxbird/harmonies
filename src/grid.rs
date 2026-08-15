use std::{prelude::*, collections::*};
use crate::util::{*, Color::*};
use crate::card::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(PartialEq)]
pub enum WaterSide {
	River,
	Islands
}

use WaterSide::*;

pub struct Grid {
	pub h: usize, pub w: usize,
	pub hexes: Vec<Vec<Hex>>,
	pub side: WaterSide
}

impl Grid {
	// standard grids are 5x5 and 7x4
	pub fn new_rect(h: usize, w: usize, side: WaterSide) -> Self {
		let mut hexes = Vec::with_capacity(h);
		for i in 0..h {
			let l = w-if i%2==1 {1} else {0};
			hexes.push(vec![Hex::empty(); l as usize]);
		}
		Grid {h, w, hexes, side}
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
		// try each possible rotation
		for r in 0..6 {
			let dx2 = x*2+(y as isize%2);
			let Some(Hex{cube: false,..}) = self.get2x(y, dx2) else {continue};
			let mut yum = true;
			for &(p, h) in map.iter() {
				if self.get2x(y+p.0, dx2+p.1) != Some(Hex{cube: false,..h}) {
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

	pub fn done(&self) -> bool {
		let mut empty = 0;
		for row in self.hexes.iter() {
			for hex in row.iter() {
				if hex.t == Empty {empty += 1;}
			}
		}
		empty <= 2
	}

	pub fn adj(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
		let iy = y as isize;
		let ix = x as isize;
		let b = iy%2;
		[(iy-1, ix+b-1), (iy-1, ix+b), (iy, ix-1),
		(iy, ix+1), (iy+1, ix+b-1), (iy-1, ix+b)].into_iter().filter(
			//|&(y, x)|grid.get(y).is_some_and(|r|r.get(x).is_some())
			|p|p.0 >= 0 && p.1 >= 0 && self.get(p.0 as usize, p.1 as usize).is_some()
		).map(
			|p|(p.0 as usize, p.1 as usize)
		).collect()
	}

	// lengths of groups of hexes that satisfy the predicate
	pub fn groups<P>(&self, pred: P) -> Vec<u32>
	where P: Fn(Hex) -> bool {
		let mut lens = Vec::new();
		let mut seen = HashSet::new();
		for (r, row) in self.hexes.iter().enumerate() {
			for (c, hex) in row.iter().enumerate() {
				if !pred(self.hexes[r][c]) || seen.contains(&(r, c)) {continue;}
				let mut q = VecDeque::new();
				let mut len = 0;
				q.push_back((r, c));
				seen.insert((r, c));
				while let Some(pos) = q.pop_front() {
					len += 1;
					for (y, x) in self.adj(pos.0, pos.1) {
						if !pred(self.hexes[y][x]) || seen.contains(&(y, x)) {
							continue;
						} 
						seen.insert((y, x));
						q.push_back((y, x));
					}
				}
				lens.push(len);
			}
		}
		lens
	}

	pub fn count<P>(&self, pred: P) -> u32
	where P: Fn(Hex) -> bool {
		let mut count = 0;
		for row in self.hexes.iter() {
			for &hex in row.iter() {
				if pred(hex) {count += 1;}
			}
		}
		count
	}

	pub fn score_river(&self) -> u32 {
		let mut best = 0;
		for (r, row) in self.hexes.iter().enumerate() {
			for (c, hex) in row.iter().enumerate() {
				if (self.hexes[r][c].t != Water) {continue;}
				let mut seen = HashSet::new();
				let mut q = VecDeque::new();
				let mut d = 1;
				q.push_back((r, c, 1));
				seen.insert((r, c));
				while let Some(pos) = q.pop_front() {
					for (y, x) in self.adj(pos.0, pos.1) {
						if self.hexes[y][x].t != Water || seen.contains(&(y, x)) {continue;} 
						q.push_back((y, x, pos.2+1));
						seen.insert((y, x));
						d = d.max(pos.2+1);
					}
				}
				best = best.max(d);
			}
		}
		[0, 0, 2, 5, 8, 11, 15][best.min(6)]+4*(best.max(6)-6) as u32
	}

	pub fn score(&self) -> BTreeMap<Tile, u32> {
		let mut map = BTreeMap::new();
		// tree scoring
		map.insert(Leaf, 0);
		for row in self.hexes.iter() {
			for hex in row {
				if hex.t == Leaf {
					*map.get_mut(&Leaf).unwrap() += [0, 1, 3, 7][hex.h as usize];	
				}
			}
		}	
		
		// mountain scoring
		map.insert(Rock, 0);
		for (r, row) in self.hexes.iter().enumerate() {
			for (c, hex) in row.iter().enumerate() {
				if hex.t != Rock {continue;}
				let mut alone = true;
				for (y, x) in self.adj(r, c) {
					if self.hexes[y][x].t == Rock {alone = false;}
				}
				if !alone {
					*map.get_mut(&Rock).unwrap() += [0, 1, 3, 7][hex.h as usize];
				}
			}
		}

		// field scoring
		map.insert(Field, 5*self.groups(|h|h.t == Field).into_iter().filter(|&n|n>1).count() as u32);

		// building scoring
		map.insert(Brick, 0);
		for (r, row) in self.hexes.iter().enumerate() {
			for (c, hex) in row.iter().enumerate() {
				if hex.t != Brick || hex.h < 2 {continue;}
				let mut colors = HashSet::new();
				for (y, x) in self.adj(r, c) {
					if self.hexes[y][x].t != Empty {colors.insert(self.hexes[y][x].t);}
				}
				if colors.len() >= 3 {
					*map.get_mut(&Brick).unwrap() += 5;
				}
			}
		}
		
		// water scoring
		map.insert(Water,
			match self.side {
				River => {self.score_river()},
				Islands => {5*self.groups(|h|h.t != Water).len() as u32}
			}
		);
		map
	}
}
