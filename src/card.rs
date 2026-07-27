use std::collections::HashMap;
use crate::{grid::*, util::*};

pub struct Card {
	pub name: String,
	pub reqs: Vec<Hex>,
	pub score: Vec<u32>,
	pub loc: usize,
	pub cubes: usize
}

impl Card {
	pub fn new(name: &str, reqs: Vec<(Tile, u8)>, loc: usize, score: Vec<u32>) -> Self {
		let mut r = reqs.iter().map(|(t, h)|{Hex::new(*t, *h)}).collect::<Vec<_>>();
		r[loc].cube = true;
		Card {name: name.to_string(), reqs: r, score, loc, cubes: 0}
	}

	pub fn show(&self) -> Vec<String> {
		let mut lines = vec![
			if use_color() {format!("╭{}{:─<12}╮",
				self.reqs[self.loc].t.color().ansi(), self.name.clone()+&color_reset()
			)}
			else {format!("╭{}{:─<8}╮",
				self.reqs[self.loc].t.color().ansi(), self.name.clone()+&color_reset()
			)},
			"│  ".to_string(),
			"│ ".to_string(),
			"╰".to_string()
		];

		for i in 0..5 {
			let l = if i<2 {1} else {2};
			lines[l] += &self.reqs[i].show();
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

use crate::Tile::*;
// TODO: add in dino promo

pub fn deck() -> Vec<Card> {
	vec![
		Card::new("Alpaca",
			vec![(Empty, 0), (Empty, 0), (Rock, 2), (Field, 1), (Field, 1)], 4,
			vec![5, 7]),
		Card::new("Bat",
			vec![(Leaf, 3), (Empty, 0), (Empty, 0), (Rock, 1), (Empty, 0)], 3,
			vec![3, 3, 4, 6]),
		Card::new("Baboon",
			vec![(Water, 1), (Water, 1), (Empty, 0), (Rock, 2), (Empty, 0)], 3,
			vec![5, 6]),
		Card::new("Bear",
			vec![(Rock, 2), (Rock, 2), (Empty, 0), (Leaf, 1), (Empty, 0)], 3,
			vec![5, 6]),
		Card::new("Bee",
			vec![(Field, 1), (Field, 1), (Field, 1), (Leaf, 2), (Empty, 0)], 3,
			vec![9, 9]), // should be 8,10 but whatever
		Card::new("Boar",
			vec![(Brick, 2), (Empty, 0), (Empty, 0), (Leaf, 2), (Empty, 0)], 3,
			vec![4, 4, 5]),
		Card::new("Croc",
			vec![(Empty, 0), (Empty, 0), (Leaf, 3), (Water, 1), (Water, 1)], 4,
			vec![4, 5, 6]),
		Card::new("Dsrtfox",
			vec![(Empty, 0), (Empty, 0), (Field, 1), (Rock, 1), (Rock, 1)], 4,
			vec![4, 5, 6]),
		Card::new("Duck",
			vec![(Brick, 2), (Empty, 0), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![2, 2, 4, 5]),
		Card::new("Flmngo",
			vec![(Field, 1), (Field, 1), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![4, 6, 6]),
		Card::new("Fish",
			vec![(Rock, 3), (Empty, 0), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![3, 3, 4, 6]),
		Card::new("Frog",
			vec![(Leaf, 1), (Empty, 0), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![2, 2, 2, 4, 5]),
		Card::new("Hdghog",
			vec![(Leaf, 2), (Leaf, 2), (Empty, 0), (Brick, 2), (Empty, 0)], 3,
			vec![5, 7]),
		Card::new("Kngfshr",
			vec![(Empty, 0), (Water, 1), (Water, 1), (Leaf, 3), (Empty, 0)], 3,
			vec![5, 6, ]),
		Card::new("Koala",
			vec![(Leaf, 1), (Empty, 0), (Empty, 0), (Leaf, 2), (Empty, 0)], 3,
			vec![3, 3, 4, 5]),
		Card::new("Ladybug",
			vec![(Leaf, 1), (Empty, 0), (Empty, 0), (Field, 1), (Empty, 0)], 3,
			vec![2, 3, 3, 4, 5]),
		Card::new("Lizard",
			vec![(Empty, 0), (Empty, 0), (Field, 1), (Field, 1), (Brick, 2)], 3,
			vec![5, 5, 6]),
		Card::new("Meerkat",
			vec![(Field, 1), (Empty, 0), (Empty, 0), (Rock, 1), (Empty, 0)], 3,
			vec![2, 3, 4, 5]),
		Card::new("Mouse",
			vec![(Empty, 0), (Field, 1), (Field, 1), (Brick, 2), (Empty, 0)], 3,
			vec![5, 5, 7]),
		Card::new("Panther",
			vec![(Empty, 0), (Empty, 0), (Leaf, 2), (Leaf, 2), (Field, 1)], 4,
			vec![5, 6]),
		Card::new("Parrot",
			vec![(Water, 1), (Water, 1), (Empty, 0), (Leaf, 2), (Empty, 0)], 3,
			vec![4, 5, 5]),
		Card::new("Peacock",
			vec![(Empty, 0), (Water, 1), (Water, 1), (Brick, 2), (Empty, 0)], 3,
			vec![5, 5, 7]),
		Card::new("Rabbit",
			vec![(Empty, 0), (Empty, 0), (Brick, 2), (Leaf, 1), (Leaf, 1)], 4,
			vec![5, 5, 7]),
		Card::new("Raccoon",
			vec![(Water, 1), (Water, 1), (Water, 1), (Field, 1), (Empty, 0)], 3,
			vec![6, 6]),
		Card::new("Raven",
			vec![(Empty, 0), (Brick, 2), (Brick, 2), (Field, 1), (Empty, 0)], 3,
			vec![4, 5]),
		Card::new("Ray",
			vec![(Rock, 1), (Rock, 1), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![4, 6, 6]),
		Card::new("Snowfox",
			vec![(Empty, 0), (Leaf, 2), (Leaf, 2), (Field, 1), (Empty, 0)], 3,
			vec![5, 5, 7]),
		Card::new("Sqrrl",
			vec![(Leaf, 3), (Empty, 0), (Empty, 0), (Brick, 2), (Empty, 0)], 3,
			vec![4, 5, 6]),
		Card::new("Otter",
			vec![(Empty, 0), (Empty, 0), (Leaf, 1), (Leaf, 1), (Water, 1)], 4,
			vec![5, 5, 6]),
		Card::new("Penguin",
			vec![(Empty, 0), (Water, 1), (Water, 1), (Rock, 1), (Empty, 0)], 3,
			vec![4, 6, 6]),
		Card::new("Vulture",
			vec![(Field, 1), (Empty, 0), (Empty, 0), (Rock, 3), (Empty, 0)], 3,
			vec![5, 6]),
		Card::new("Wolf",
			vec![(Field, 1), (Field, 1), (Empty, 0), (Leaf, 3), (Empty, 0)], 3,
			vec![4, 6, 6]),
	]
}

// the soul animals or whatever
pub fn souls() -> Vec<Card> {
	vec![
		Card::new("Bttrfly",
			vec![(Water, 1), (Field, 1), (Field, 1), (Water, 1), (Empty, 0)], 1,
			vec![]),
		Card::new("Cat",
			vec![(Empty, 0), (Empty, 0), (Brick, 2), (Leaf, 1), (Brick, 2)], 4,
			vec![]),
		Card::new("Deer",
			vec![(Empty, 0), (Empty, 0), (Leaf, 3), (Leaf, 2), (Leaf, 1)], 3,
			vec![]),
		Card::new("Drgnfly",
			vec![(Empty, 0), (Empty, 0), (Leaf, 2), (Water, 1), (Leaf, 2)], 3,
			vec![]),
		Card::new("Goat",
			vec![(Rock, 2), (Empty, 0), (Empty, 0), (Rock, 3), (Empty, 0)], 3,
			vec![]),
		Card::new("Lion",
			vec![(Empty, 0), (Empty, 0), (Leaf, 2), (Field, 1), (Field, 1)], 3,
			vec![]),
		Card::new("Marmot",
			vec![(Empty, 0), (Rock, 1), (Rock, 1), (Rock, 2), (Empty, 0)], 3,
			vec![]),
		Card::new("Stork",
			vec![(Empty, 0), (Empty, 0), (Field, 1), (Brick, 2), (Brick, 2)], 4,
			vec![]),
		Card::new("Tortse",
			vec![(Empty, 0), (Empty, 0), (Rock, 2), (Water, 1), (Water, 1)], 3,
			vec![]),
		Card::new("Owl",
			vec![(Leaf, 1), (Leaf, 1), (Empty, 0), (Leaf, 3), (Empty, 0)], 3,
			vec![]),
	]
}

// animals from the pulse expansion
pub fn pulse() -> Vec<Card> {
	vec![
		Card::new("Prnha",
			vec![(Water, 1), (Empty, 0), (Empty, 0), (Leaf, 3), (Empty, 0)], 0,
			vec![6, 7]),
		Card::new("Axolotl",
			vec![(Rock, 2), (Leaf, 2), (Empty, 0), (Water, 1), (Empty, 0)], 3,
			vec![7, 9]),
		//TODO: check name
		Card::new("Wren?",
			vec![(Rock, 1), (Leaf, 1), (Empty, 0), (Brick, 2), (Empty, 0)], 3,
			vec![4, 6, 7]),
		Card::new("Gull",
			vec![(Brick, 2), (Empty, 0), (Empty, 0), (Field, 1), (Water, 1)], 0,
			vec![5, 6]),
		Card::new("Tiger",
			vec![(Leaf, 1), (Empty, 0), (Empty, 0), (Rock, 1), (Water, 1)], 0,
			vec![3, 7, 6]),
		Card::new("Chmln",
			vec![(Field, 1), (Empty, 0), (Empty, 0), (Leaf, 1), (Water, 1)], 3,
			vec![4, 4, 7]),
		Card::new("Camel",
			vec![(Field, 1), (Empty, 0), (Empty, 0), (Rock, 1), (Brick, 2)], 3,
			vec![4, 5, 7]),
		Card::new("Yak",
			vec![(Empty, 0), (Empty, 0), (Water, 1), (Rock, 2), (Field, 1)], 3,
			vec![7, 9]),
		Card::new("Panda",
			vec![(Rock, 1), (Leaf, 3), (Empty, 0), (Field, 1), (Empty, 0)], 3,
			vec![5, 8]),
		Card::new("Beetle",
			vec![(Empty, 0), (Empty, 0), (Brick, 2), (Field, 1), (Rock, 1)], 3,
			vec![5, 7])
	]
}

// TODO: keep an eye out for crescendo expansion
