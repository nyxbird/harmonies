use std::mem;
use std::{io::*, prelude::*};
use crate::{draw::*, game::*, grid::*};

pub fn prompt(m: u8) -> String {
	margin(m);
	print!("> ");
	stdout().flush();
	let mut inp = String::new();
	stdin().read_line(&mut inp).unwrap();
	inp.trim().to_string()
}

pub fn pick_pool(g: &mut Game, m: u8) -> bool {
	margin(m);
	print!("> ");
	stdout().flush();
	let mut inp = String::new();
	stdin().read_line(&mut inp).unwrap();
	let Ok(p) = inp.trim().parse::<usize>()
		else {return false};
	if p < 0 || p > 2 {return false;}
	g.held = mem::take(&mut g.pools[p]);
	true
}

pub fn place_hex(g: &mut Game, m: u8) -> Option<()> {
	let s = prompt(m).chars().collect::<Vec<_>>();
	if s.len() != 3 {return None;}
	if s[0] < '1' || s[0] > '3' {return None;}
	let hex = mem::take(&mut g.held[s[0] as usize-'1' as usize]);
	if s[1] < 'a' || s[2] < '0' {return None;}
	let x = s[1] as usize-'a' as usize;
	let y = s[2] as usize-'1' as usize;
	g.grid.stack(y, x, &hex);
	Some(())
}

pub fn place_cube(g: &mut Game, m: u8) -> Option<()> {
	let s = prompt(m).chars().collect::<Vec<_>>();
	if s.len() != 3 {return None;}
	if s[0] < 'a' {return None;}
	let c = &g.shop[s[0] as usize-'a' as usize];
	if s[1] < 'a' || s[2] < '0' {return None;}
	let x = s[1] as isize-'a' as isize;
	let y = s[2] as isize-'1' as isize;
	if g.grid.valid_cube(y, x, c) {
		println!("!!!!yes!!!!");
	}
	Some(())
}

pub fn ask(g: &mut Game, m: u8) -> Option<()> {
	let s = prompt(m).chars().collect::<Vec<_>>();
	if s.len() == 1 {
		if s[0] == 'p' {g.new_pools();}
		else if s[0] < '1' || s[0] > '3' {return None;}
		else {g.held = mem::take(&mut g.pools[s[0] as usize-'1' as usize]);}
	}
	else if s.len() == 3 {
		let x = s[1] as usize-'a' as usize;
		let y = s[2] as usize-'1' as usize;
		if s[0] >= '1' && s[0] <= '3' {
			let hex = mem::take(&mut g.held[s[0] as usize-'1' as usize]);
			g.grid.stack(y, x, &hex);
		}
		else if s[0] >= 'a' {
			let c = &mut g.shop[s[0] as usize-'a' as usize];
			if g.grid.valid_cube(y as isize, x as isize, c) {
				g.score += (*c).score[(*c).cubes];
				(*c).cubes += 1;
				if (*c).cubes >= (*c).score.len() {
					g.shop.remove(s[0] as usize-'a' as usize);
					if let Some(n) = g.deck.pop() {
						g.shop.push(n);
					}
				}
			}
			else {return None;}
		} 
	}
	else {return None;}
	Some(())
}


