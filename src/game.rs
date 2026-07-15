use std::prelude::*;
use rand::prelude::*;
use crate::grid::*;

pub fn new_bag() -> Vec<Hex> {
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
