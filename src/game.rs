use std::{prelude::*, collections::*};
use rand::prelude::*;
use crate::{card::*, grid::*};

// current game state
pub struct Game {
	pub bag: Vec<Hex>,
	pub counts: BTreeMap<Hex, u8>,
	pub pools: Vec<Vec<Hex>>,
	pub held: Vec<Hex>,
	pub deck: Vec<Card>,
	pub shop: Vec<Card>,
	pub grid: Grid,
	pub owned: Vec<Card>,
	pub score: u32,
}

impl Game {
	pub fn new() -> Game {
		let mut counts = BTreeMap::from([
			(Hex::Wood, 21), (Hex::Leaf, 19),
			(Hex::Rock, 23), (Hex::Brick, 15),
			(Hex::Field, 19), (Hex::Water, 23)
		]);
		let mut bag = Vec::new();
		for (&h, &c) in counts.iter() {
			for _ in 0..c {bag.push(h);}
		}
		let mut rng = rand::rng();
		bag.shuffle(&mut rng);
		let pools = take_pools(&mut bag).unwrap();
		let mut deck = deck();
		deck.shuffle(&mut rng);
		let mut shop = vec![];
		for _ in 0..4 {shop.push(deck.pop().unwrap());}
		Game {
			bag, counts, pools,
			held: Vec::new(),
			deck, shop,
			grid: Grid::new_rect(5, 5),
			owned: Vec::new(),
			score: 0
		}
	}

	pub fn new_pools(&mut self) -> bool {
		self.pools = vec![Vec::new(); 3];
		for p in 0..3 {
			for _ in 0..3 {
				let Some(h) = self.bag.pop() else {return false};
				self.pools[p].push(h);
			}
		}
		true
	}
}

pub fn new_bag() -> (Vec<Hex>) {
	let mut bag = Vec::new();
	for _ in 0..21 {bag.push(Hex::Wood);}
	for _ in 0..19 {bag.push(Hex::Leaf);}
	for _ in 0..23 {bag.push(Hex::Rock);}
	for _ in 0..15 {bag.push(Hex::Brick);}
	for _ in 0..19 {bag.push(Hex::Field);}
	for _ in 0..23 {bag.push(Hex::Water);}
	let mut rng = rand::rng();
	bag.shuffle(&mut rng);
	bag
}

pub fn take_pools(bag: &mut Vec<Hex>) -> Option<Vec<Vec<Hex>>> {
	let mut pools = vec![Vec::new(); 3];
	for p in 0..3 {
		for _ in 0..3 {
			pools[p].push(bag.pop()?);
		}
		pools[p].sort();
	}
	Some(pools)
}


