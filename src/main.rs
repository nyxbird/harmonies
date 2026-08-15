#![allow(warnings)]

mod grid;
mod draw;
mod game;
mod user;
mod card;
mod util;
use std::prelude::*;
use rand::prelude::*;
use clap::Parser;
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

#[derive(Parser)]
#[command(name = "Harmonies")]
#[command(version = "0.0")]
#[command(about = "A tile-laying game about making habitats", long_about = None)]
struct Cli {
	#[arg(short)]
	/// Play on the A-side board
	a_side: bool,
	#[arg(short)]
	/// Play on the B-side board
	b_side: bool,
	#[arg(short, long)]
	/// Include Spirit Animals
	spirit: bool,
	#[arg(short, long)]
	/// Include the Pulse expansion
	pulse: bool,
}

fn main() {
	let cli = Cli::parse();
	
	let a_side =
		if cli.a_side == cli.b_side {rand::random()}
		else {cli.a_side};
	
	let mut game =
		if a_side {Game::new(5, 5, WaterSide::River, cli.pulse)}
		else {Game::new(7, 4, WaterSide::Islands, cli.pulse)};

	// run game
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
