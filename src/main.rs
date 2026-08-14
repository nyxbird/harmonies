#![allow(warnings)]

mod grid;
mod draw;
mod game;
mod user;
mod card;
mod util;
use std::prelude::*;
use rand::prelude::*;
use crate::{grid::*, draw::*, game::*, card::*, util::*, user::*};

// TODO:
// cli options:
// grid side (A: (5x5 rivers), B: (7x4 islands))
// - include fanmade?
// - crescedo (when it comes out)
// spirits (random, chosen, none)
// expansions
// - dino
// - pulse
// - crescendo (when it comes out)
// melodies (random, selected)
// high score tracker
// - just write to a file with options?
// - in-game viewer?

fn main() {
	let mut game = Game::new(5, 5, WaterSide::Islands);
	while(!game.done) {
		print!("{}", clear());
		draw(&game, 3);
		while !ask(&mut game, 3) {
			println!("invalid");
		}
	}
	print!("{}", clear());
	draw(&game, 3);
	end(&mut game);
}
