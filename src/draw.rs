use std::prelude::*;
use crate::{game::*, grid::*, util::*};

impl Grid {
	pub fn draw(&self) {
		let lines = self.show();
		for line in lines {
			println!("{}", line);
		}
	}

	pub fn show(&self) -> Vec<String> {
		let mut lines = vec![String::new(); self.h+1];
		lines[0] += "  ";
		for i in 0..self.w {
			lines[0].push(' ');
			lines[0].push((b'a'+i as u8) as char);
		}
		for i in 1..self.h+1 {
			lines[i] += &format!("{} ", i);
			if i%2 == 0 {lines[i].push(' ');}
			for (j, &hex) in self.hexes[i-1].iter().enumerate() {
				lines[i] += &hex.show();
			}
			if i%2 == 0 {lines[i].push(' ');}
		}
		lines
	}
}

pub fn margin(m: u8) {
	for _ in 0..m {print!(" ");}
}

pub fn draw(g: &Game, m: u8) {
	// draw relevant info
	let mut score_sum = g.score+g.score_spirit();
	let scores = g.grid.score();
	for (_, &s) in scores.iter() {score_sum += s;}
	margin(m);
	print!("score: {} [{}] ({}", score_sum, g.rating(score_sum), g.score);
	for (&t, &s) in scores.iter() {
		print!("{}+{}{}", t.color().ansi(), s, Color::Norm.ansi());
	}
	if g.spirit.is_some() {
		print!("+{}", g.score_spirit());
	}
	println!(")");

	let mut count_sum = 0;
	for (_, &c) in g.counts.iter() {count_sum += c;}
	margin(m);
	print!("rem: {} (", count_sum);
	for (i, (t, c)) in g.counts.iter().enumerate() {
		print!("{}{} {}", t.color().ansi(), c, t.show());
		if i != g.counts.len()-1 {print!(", ");}
	}
	println!(")");

	margin(m);
	for _ in 0..9 {print!("─");}
	print!("┬");
	for _ in 0..11*g.shop.len()+3 {print!("─");}
	println!();

	// draw pools and cards
	let mut shop = vec![];
	for c in g.shop.iter() {
		shop.push(c.show());
	}
	for i in 0..4 {
		margin(m);
		if g.held.len() == 0 {
			if i == 0 {print!("pools:  ");}
			else {
				print!("{}:", i);
				for j in 0..3 {
					print!(" {}", g.pools[i-1].get(j).unwrap_or(&Tile::Empty).show());
				}
			}
		}
		else {
			if i == 1 {print!("  held: ");}
			else if i == 2 {
				print!("  ");
				for t in 0..3 {
					print!("{} ",
						if t < g.held.len() {g.held[t].show()}
						else {" ".to_string()}
					);
				}
			}
			else {print!("        ");}
		}
		print!(" │");
		if i == 0 && !g.cardact {print!("*");} else {print!(" ");}
		for c in shop.iter() {
			print!("{} ", c[i]);
		}
		println!();
	}
	margin(m);
	for i in 0..11*g.shop.len()+13 {
		let x = (if i == 9 {2} else {0}) + (if i == 2*g.grid.w+3 {1} else {0});
		print!("{}", ["─", "┬", "┴", "┼"][x]);
	}
	println!();
	
	// draw board and owned cards
	let grid = g.grid.show();
	let mut owned = vec![];
	for c in g.owned.iter() {
		owned.push(c.show());
	}
	for i in 0..grid.len().max(4) {
		margin(m);
		// TODO: bug if grid height is too small
		print!("{} │ ", grid[i]);
		if i < 4 {	
			for c in owned.iter() {
				print!("{} ", c[i]);
			}
		}
		println!();
	}
}

