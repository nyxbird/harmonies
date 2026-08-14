use std::mem;
use std::{io::*, prelude::*};
use crate::{draw::*, card::*, game::*, grid::*, util::*};

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

pub fn pref_filter(pre: &[char], cards: &Vec<Card>) -> Option<usize> {
	let s = pre.iter().collect::<String>();
	let mut res = None;
	for (i, card) in cards.iter().enumerate() {
		if card.name.to_lowercase().starts_with(&s) {
			println!("match!");
			res = if res == None {Some(i)} else {return None};
		}
	}
	res
}

// potential inputs:
// 1-3: take a pool
// t/l/r/b/f/w+[yx]: place a tile from held
// \w{3,}: take a card from the shop
// \w{3,}[yx]: place a cube from owned cards
// !\w{3,}/\w{3}!: trash a card from the shop
pub fn ask(g: &mut Game, m: u8) -> bool {
	let arr = prompt(m).chars().collect::<Vec<_>>();
	let s = arr.as_slice();
	match s {
		['q'] => {g.done = true;},
		[c@'1'..='3'] => {
			// pools 
			if !g.held.is_empty() {return false;}
			g.held = g.pools.swap_remove(*c as usize-'1' as usize);
			g.new_pools();
		},
		[c@('t'|'l'|'r'|'b'|'f'|'w'), xc@'a'..='j', yc@'1'..='9'] => {
			// place tile from held
			//let tile = mem::take(&mut g.held[s[0] as usize-'1' as usize]);
			let mut tile = None;
			for i in 0..g.held.len() {
				if g.held[i].symbol() == *c {
					tile = Some(g.held[i]);
					g.held.remove(i);
					break;
				}
			}
			if tile == None {return false}
			let y = *yc as usize-'1' as usize; let x = *xc as usize-'a' as usize;
			g.grid.stack(y, x, &tile.unwrap());
			if g.grid.done() {g.done = true;}
			if g.held.len() == 0 {g.cardact = true;}
		},
		['!', p@..]|[p@.., '!'] => {
			// trash from shop			
			if g.cardact {return false;}
			let Some(i) = pref_filter(p, &g.shop) else {return false};
			g.shop.remove(i);
			if let Some(c) = g.deck.pop() {
				g.shop.push(c);
			}
			g.cardact = false;
		},
		[xc@'a'..='j', yc@'1'..='9', p@..]|[p@.., xc@'a'..='j', yc@'1'..='9'] => {
			let Some(i) = pref_filter(p, &g.owned) else {return false};
			let c = g.owned.get_mut(i).unwrap();
			let y = *yc as usize-'1' as usize; let x = *xc as usize-'a' as usize;
			if g.grid.valid_cube(y as isize, x as isize, c) {
				g.grid.hexes[y][x].cube = true;
				if let Score::Cubes(pts) = &(*c).score {
					g.score += pts[(*c).cubes];
				}
				(*c).cubes += 1;
				if (*c).done() {
					g.owned.remove(i);
				}
			}
			else {return false;}
		},
		_ => {
			// take from shop
			if g.cardact {return false;}
			let Some(i) = pref_filter(s, &g.shop) else {return false};
			if g.owned.len() >= 4 || g.shop.len() <= i {return false}
			g.owned.push(g.shop.remove(i));
			if let Some(c) = g.deck.pop() {
				g.shop.push(c);
			}
			g.cardact = true;
		}
	}
	true
}

pub fn end(g: &mut Game) {
	println!("complete!");
	let mut total = g.score;
	for v in g.grid.score().values() {
		total += v;
	}
	print!("final score: {}! ({}", total, g.score);
	for (t, v) in g.grid.score() {
		print!("{}+{}{}", t.color().ansi(), v, Color::Norm.ansi());
	}
	println!(")");
}

//pub fn ask_() {
//	let s = prompt(m).chars().collect::<Vec<_>>();
//	if s.len() == 1 {
//		match s[0] {
//			'p' => {g.new_pools();}
//			'q' => {g.done = true;}
//			'1'..'3' => {
//				g.held = mem::take(&mut g.pools[s[0] as usize-'1' as usize]);
//			}
//			'a'..'d' => {
//				if g.owned.len() >= 4 || g.shop.len() <= s[0] as usize-'a' as usize {return None;}
//				g.owned.push(g.shop.remove(s[0] as usize-'a' as usize));
//				if let Some(c) = g.deck.pop() {
//					g.shop.push(c);
//				}
//			}
//			_ => {return None;}
//		}
//	}
//	else if s.len() == 3 {
//		let x = s[1] as usize-'a' as usize;
//		let y = s[2] as usize-'1' as usize;
//		if s[0] >= '1' && s[0] <= '3' {
//			let tile = mem::take(&mut g.held[s[0] as usize-'1' as usize]);
//			g.grid.stack(y, x, &tile);
//		}
//		else if s[0] >= 'a' {
//			let Some(c) = g.owned.get_mut(s[0] as usize-'a' as usize)
//				else {return None};
//			if g.grid.valid_cube(y as isize, x as isize, c) {
//				g.grid.hexes[y][x].cube = true;
//				g.score += (*c).score[(*c).cubes];
//				(*c).cubes += 1;
//				if (*c).cubes >= (*c).score.len() {
//					g.owned.remove(s[0] as usize-'a' as usize);
//				}
//			}
//			else {return None;}
//		} 
//	}
//	else {return None;}
//	Some(())
//}
