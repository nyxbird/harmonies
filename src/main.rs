#![allow(warnings)]

mod grid;
mod draw;
mod game;
mod card;
mod util;
use std::prelude::*;
use crate::{grid::*, draw::*, game::*};

fn main() {
	//let g = Grid::new(5, 5);
	let g = Grid::new_rect(5, 5);
	g.draw();
	let mut bag = new_bag();
	let pools = take_pools(&mut bag).unwrap();
	for pool in pools.iter() {
		for hex in pool {
			print!("{} ", hex.show());
		}
		println!();
	}
}
