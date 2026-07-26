use std::collections::HashMap;
use crate::{grid::*, util::*};

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

pub struct Card {
	pub name: String,
	//pub reqs: HashMap<Pos, Hex>,
	pub reqs: Vec<Hex>,
	pub loc: usize,
	pub score: Vec<u32>,
	pub cubes: usize
}

impl Card {
	//pub fn new(name: String, reqs: HashMap<Pos, Hex>, score: Vec<u32>) -> Self {
	pub fn new(name: &str, reqs: Vec<Hex>, loc: usize, score: Vec<u32>) -> Self {
		Card {name: name.to_string(), reqs, loc, score, cubes: 0}
	}	

	pub fn show(&self) -> Vec<String> {
		let mut lines = vec![
			if use_color() {format!("╭{}{:─<12}╮",
				self.reqs[self.loc].color().ansi(), self.name.clone()+&color_reset()
			)}
			else {format!("╭{}{:─<8}╮",
				self.reqs[self.loc].color().ansi(), self.name.clone()+&color_reset()
			)},
			"│  ".to_string(),
			"│ ".to_string(),
			"╰".to_string()
		];

		for i in 0..5 {
			let l = if i<2 {1} else {2};
			lines[l] += &self.reqs[i].show(self.loc==i);
		}
		lines[1] += "  │"; lines[2] += " │";

		for i in 0..5 {
			lines[3].push(
				if self.score.len() > i {
					if self.cubes <= i {char::from_digit(self.score[i], 10).unwrap()}
					else {'╌'}	
				}
				else if i == 4 {'╯'} else {'─'}
			);
			if i < 4 {lines[3].push('─')};
		}

		if self.score.len() == 0 {
			lines[3] = "╘════════╛".to_string();
		}
		lines
	}
}

use crate::Hex::*;
// TODO: add in dino promo

pub fn deck() -> Vec<Card> {
	vec![
		Card::new("Alpaca",
			vec![Empty, Empty, TwoRock, Field, Field], 4,
			vec![5, 7]),
		Card::new("Bat",
			vec![TwoTree, Empty, Empty, Rock, Empty], 3,
			vec![3, 3, 4, 6]),
		Card::new("Baboon",
			vec![Water, Water, Empty, TwoRock, Empty], 3,
			vec![5, 6]),
		Card::new("Bear",
			vec![TwoRock, TwoRock, Empty, Leaf, Empty], 3,
			vec![5, 6]),
		Card::new("Bee",
			vec![Field, Field, Field, Tree, Empty], 3,
			vec![9, 9]), // should be 8,10 but whatever
		Card::new("Boar",
			vec![Building, Empty, Empty, Tree, Empty], 3,
			vec![4, 4, 5]),
		Card::new("Bttrfly",
			vec![Water, Field, Field, Water, Empty], 1,
			vec![]),
		Card::new("Cat",
			vec![Empty, Empty, Building, Leaf, Building], 4,
			vec![]),
		Card::new("Croc",
			vec![Empty, Empty, TwoTree, Water, Water], 4,
			vec![4, 5, 6]),
		Card::new("Deer",
			vec![Empty, Empty, TwoTree, Tree, Leaf], 3,
			vec![]),
		Card::new("Dsrtfox",
			vec![Empty, Empty, Field, Rock, Rock], 4,
			vec![4, 5, 6]),
		Card::new("Drgnfly",
			vec![Empty, Empty, Tree, Water, Tree], 3,
			vec![]),
		Card::new("Duck",
			vec![Building, Empty, Empty, Water, Empty], 3,
			vec![2, 2, 4, 5]),
		Card::new("Flmngo",
			vec![Field, Field, Empty, Water, Empty], 3,
			vec![4, 6, 6]),
		Card::new("Fish",
			vec![ThreeRock, Empty, Empty, Water, Empty], 3,
			vec![3, 3, 4, 6]),
		Card::new("Frog",
			vec![Leaf, Empty, Empty, Water, Empty], 3,
			vec![2, 2, 2, 4, 5]),
		Card::new("Goat",
			vec![TwoRock, Empty, Empty, ThreeRock, Empty], 3,
			vec![]),
		Card::new("Hdghog",
			vec![Tree, Tree, Empty, Building, Empty], 3,
			vec![5, 7]),
		Card::new("Kngfshr",
			vec![Empty, Water, Water, TwoTree, Empty], 3,
			vec![5, 6, ]),
		Card::new("Koala",
			vec![Leaf, Empty, Empty, Tree, Empty], 3,
			vec![3, 3, 4, 5]),
		Card::new("Ladybug",
			vec![Leaf, Empty, Empty, Field, Empty], 3,
			vec![2, 3, 3, 4, 5]),
		Card::new("Lion",
			vec![Empty, Empty, Tree, Field, Field], 3,
			vec![]),
		Card::new("Lizard",
			vec![Empty, Empty, Field, Field, Building], 3,
			vec![5, 5, 6]),
		Card::new("Marmot",
			vec![Empty, Rock, Rock, TwoRock, Empty], 3,
			vec![]),
		Card::new("Meerkat",
			vec![Field, Empty, Empty, Rock, Empty], 3,
			vec![2, 3, 4, 5]),
		Card::new("Mouse",
			vec![Empty, Field, Field, Building, Empty], 3,
			vec![5, 5, 7]),
		Card::new("Panther",
			vec![Empty, Empty, Tree, Tree, Field], 4,
			vec![5, 6]),
		Card::new("Parrot",
			vec![Water, Water, Empty, Tree, Empty], 3,
			vec![4, 5, 5]),
		Card::new("Peacock",
			vec![Empty, Water, Water, Building, Empty], 3,
			vec![5, 5, 7]),
		Card::new("Rabbit",
			vec![Empty, Empty, Building, Leaf, Leaf], 4,
			vec![5, 5, 7]),
		Card::new("Raccoon",
			vec![Water, Water, Water, Field, Empty], 3,
			vec![6, 6]),
		Card::new("Raven",
			vec![Empty, Building, Building, Field, Empty], 3,
			vec![4, 5]),
		Card::new("Ray",
			vec![Rock, Rock, Empty, Water, Empty], 3,
			vec![4, 6, 6]),
		Card::new("Snowfox",
			vec![Empty, Tree, Tree, Field, Empty], 3,
			vec![5, 5, 7]),
		Card::new("Sqrrl",
			vec![TwoTree, Empty, Empty, Building, Empty], 3,
			vec![4, 5, 6]),
		Card::new("Stork",
			vec![Empty, Empty, Field, Building, Building], 4,
			vec![]),
		Card::new("Tortse",
			vec![Empty, Empty, TwoRock, Water, Water], 3,
			vec![]),
		Card::new("Otter",
			vec![Empty, Empty, Leaf, Leaf, Water], 4,
			vec![5, 5, 6]),
		Card::new("Owl",
			vec![Leaf, Leaf, Empty, TwoTree, Empty], 3,
			vec![]),
		Card::new("Penguin",
			vec![Empty, Water, Water, Rock, Empty], 3,
			vec![4, 6, 6]),
		Card::new("Vulture",
			vec![Field, Empty, Empty, ThreeRock, Empty], 3,
			vec![5, 6]),
		Card::new("Wolf",
			vec![Field, Field, Empty, TwoTree, Empty], 3,
			vec![4, 6, 6]),
	]
}

// animals from the pulse expansion
pub fn pulse() -> Vec<Card> {
	vec![
		Card::new("Prnha",
			vec![Water, Empty, Empty, TwoTree, Empty], 0,
			vec![6, 7]),
		Card::new("Axolotl",
			vec![TwoRock, Tree, Empty, Water, Empty], 3,
			vec![7, 9]),
		//TODO: check name
		Card::new("Wren?",
			vec![Rock, Leaf, Empty, Building, Empty], 3,
			vec![4, 6, 7]),
		Card::new("Gull",
			vec![Building, Empty, Empty, Field, Water], 0,
			vec![5, 6]),
		Card::new("Tiger",
			vec![Leaf, Empty, Empty, Rock, Water], 0,
			vec![3, 7, 6]),
		Card::new("Chmln",
			vec![Field, Empty, Empty, Leaf, Water], 3,
			vec![4, 4, 7]),
		Card::new("Camel",
			vec![Field, Empty, Empty, Rock, Building], 3,
			vec![4, 5, 7]),
		Card::new("Yak",
			vec![Empty, Empty, Water, TwoRock, Field], 3,
			vec![7, 9]),
		Card::new("Panda",
			vec![Rock, TwoTree, Empty, Field, Empty], 3,
			vec![5, 8]),
		Card::new("Beetle",
			vec![Empty, Empty, Building, Field, Rock], 3,
			vec![5, 7])
	]
}

// TODO: keep an eye out for crescendo expansion
