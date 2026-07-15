use std::prelude::*;
use crate::util::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hex {
	// Blank is a non-space for grid shaping
	Blank, Empty,
	Wood, TwoWood,
	Leaf, Tree, TwoTree,
	Rock, TwoRock, ThreeRock,
	Brick, Building,
	Field,
	Water
}

impl Hex {
	//use Hex::*;
	//use crate::util::Color::*;
	// stack another hex on this one
	pub fn stack(&self, top: &Hex) -> Option<Hex> {
		match (*self, *top) {
			(Hex::Empty, x) => Some(x),
			(Hex::Wood, Hex::Wood) => Some(Hex::TwoWood),
			(Hex::Wood, Hex::Leaf) => Some(Hex::Tree),
			(Hex::TwoWood, Hex::Leaf) => Some(Hex::TwoTree),
			(Hex::Rock, Hex::Rock) => Some(Hex::TwoRock),
			(Hex::TwoRock, Hex::Rock) => Some(Hex::ThreeRock),
			(Hex::Wood|Hex::Rock|Hex::Brick, Hex::Brick) => Some(Hex::Building),
			_ => None
		}
	}

	pub fn color(&self) -> Color {
	use Hex::*;
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
	
	//pub fn show(&self) -> String {
	//	(match self {
	//		Hex::Blank => " ".to_string(),
	//		Hex::Empty => ".".to_string(),
	//		Hex::Wood => color_magenta()+"w",
	//		Hex::TwoWood => color_magenta()+"W",
	//		Hex::Leaf => color_green()+"l",
	//		Hex::Tree => color_green()+"L",
	//		Hex::TwoTree => color_green()+"T",
	//		Hex::Rock => color_cyan()+"r",
	//		Hex::TwoRock => color_cyan()+"R",
	//		Hex::ThreeRock => color_cyan()+"M",
	//		Hex::Brick => color_red()+"b",
	//		Hex::Building => color_red()+"B",
	//		Hex::Field => color_yellow()+"f",
	//		Hex::Water => color_blue()+"~"
	//	})+&color_reset()
	//}
	
	pub fn show(&self) -> String {
	use Hex::*;
	use crate::util::Color::*;
		self.color().ansi()+(match self {
			Blank => " ",
			Empty => ".",
			Wood => "w",
			TwoWood => "W",
			Leaf => "l",
			Tree => "L",
			TwoTree => "T",
			Rock => "r",
			TwoRock => "R",
			ThreeRock => "M",
			Brick => "b",
			Building => "B",
			Field => "f",
			Water => "~"
		})+&color_reset()
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
