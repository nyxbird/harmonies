use std::{prelude::*, collections::*};
use rand::prelude::*;
use crate::{card::*, grid::*};

pub struct State {
	pub done: bool,
	pub quit: bool,
	pub cardact: bool,
}

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
	pub cardact: bool,
	pub spirit: Option<Score>,
	pub suns: usize
}

use Tile::*;

impl Game {
	pub fn new(y: usize, x: usize, side: WaterSide, use_pulse: bool) -> Game {
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
		let mut deck = deck();
		if use_pulse {deck.append(&mut pulse());}
		deck.shuffle(&mut rng);
		let mut shop = vec![];
		for _ in 0..4 {shop.push(deck.pop().unwrap());}
		let mut g = Game {
			bag, counts, pools: vec![],
			held: Vec::new(),
			deck, shop,
			grid: Grid::new_rect(y, x, side),
			owned: Vec::new(),
			score: 0, done: false,
			cardact: false,
			spirit: None, suns: 2 
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
		
	pub fn score_spirit(&self) -> u32 {
		if let Some(Score::Spirit(f, _, _)) = self.spirit {
			f(&self.grid)
		} else {0}
	}

	pub fn rating(&self, score: u32) -> String {
		let mut suns = self.suns;
		let sum = self.score+self.grid.score().values().sum::<u32>();
		for v in [40, 70, 90, 110, 130, 140, 150, 160] {
			if sum >= v {suns += 1;}
			else {break;}
		}
		if self.grid.side == WaterSide::Islands {suns += 1;}
		["F", "D", "C", "C+",
		"B", "B+", "A", "A+",
		"S", "S+", "H", "H+"][suns].to_string()
	}
}
