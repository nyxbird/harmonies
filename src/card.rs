use std::collections::HashMap;
use crate::grid::*;

#[derive(Copy, Clone, PartialEq, Hash)]
struct Pos {
	pub x: u32,
	pub y: u32
}

impl Pos {
	pub fn new(x: u32, y: u32) -> Self {
		Pos {x, y}
	}
}

struct Card {
	pub name: String,
	pub reqs: HashMap<Pos, Hex>,
	pub score: Vec<u32>,
	pub cubes: usize
}

impl Card {
	pub fn new(name: String, reqs: HashMap<Pos, Hex>, score: Vec<u32>) -> Self {
		Card {name, reqs, score, cubes: 0}
	}	

	pub fn show(&self) -> Vec<String> {
		let mut lines = Vec::new();
		
		lines
	}
}

// Vulture
//  f M*

// Wolf
// {f f} 
// { T }
//
// Peacock
// { B ~}
// {~   }
// [5/10/17]

// {t t F}
// { }
// { }
//  . .
// . .
//
// 

