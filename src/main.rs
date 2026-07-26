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

fn main() {
	let mut game = Game::new();
	draw(&game, 3);
	loop {
		if ask(&mut game, 3) == None {
			println!("invalid");
		}
		else {
			clear();
			draw(&game, 3);
		}
	}
	//draw(&game, 3);
	//loop {
	//	if pick_pool(&mut game, 3) {
	//		println!("took pool");
	//		break;
	//	}
	//	else {println!("invalid pool");}
	//}
	//clear();
	//draw(&game, 3);
	//loop {
	//	if let Some(_) = place_hex(&mut game, 3) {
	//		println!("placed hex");
	//		break;
	//	}
	//	else {println!("invalid placement");}
	//}
	//clear();
	//draw(&game, 3);
}
