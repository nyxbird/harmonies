use std::{prelude::*, collections::*};
use rand::prelude::*;
use crate::{card::*, grid::*};

// current game state
pub struct Game {
	pub bag: Vec<Tile>,
	pub counts: BTreeMap<Tile, u8>,
	pub pools: Vec<Vec<Tile>>,
	pub held: Vec<Tile>,
	pub deck: Vec<Card>,
	pub shop: Vec<Card>,
	pub grid: Grid,
	pub owned: Vec<Card>,
	pub score: u32,
	pub done: bool,
}

use Tile::*;

impl Game {
	pub fn new() -> Game {
		let mut counts = BTreeMap::from([
			(Wood, 21), (Leaf, 19),
			(Rock, 23), (Brick, 15),
			(Field, 19), (Water, 23)
		]);
		let mut bag = Vec::new();
		for (&h, &c) in counts.iter() {
			for _ in 0..c {bag.push(h);}
		}
		let mut rng = rand::rng();
		bag.shuffle(&mut rng);
		//let pools = take_pools(&mut bag).unwrap();
		let mut deck = deck();
		deck.shuffle(&mut rng);
		let mut shop = vec![];
		for _ in 0..4 {shop.push(deck.pop().unwrap());}
		let mut g = Game {
			bag, counts, pools: vec![],
			held: Vec::new(),
			deck, shop,
			grid: Grid::new_rect(5, 5),
			owned: Vec::new(),
			score: 0, done: false
		};
		g.new_pools();
		g
	}

	pub fn new_pools(&mut self) -> bool {
		self.pools = vec![Vec::new(); 3];
		for p in 0..3 {
			for _ in 0..3 {
				let Some(h) = self.pull_tile() else {return false};
				self.pools[p].push(h);
			}
		}
		true
	}

	pub fn pull_tile(&mut self) -> Option<Tile> {
		let Some(t) = self.bag.pop() else {return None};
		*self.counts.get_mut(&t).unwrap() -= 1;
		Some(t)
	}
}

pub fn take_pools(bag: &mut Vec<Tile>) -> Option<Vec<Vec<Tile>>> {
	let mut pools = vec![Vec::new(); 3];
	for p in 0..3 {
		for _ in 0..3 {
			pools[p].push(bag.pop()?);
		}
		pools[p].sort();
	}
	Some(pools)
}
