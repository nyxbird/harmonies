use std::io::*;
use crate::{draw::*, card::*, game::*};

pub fn prompt(m: u8) -> String {
	margin(m);
	print!("> ");
	let _ = stdout().flush();
	let mut inp = String::new();
	stdin().read_line(&mut inp).unwrap();
	inp.trim().to_string()
}

pub fn pref_filter(pre: &[char], cards: &[Card]) -> Option<usize> {
	let s = pre.iter().collect::<String>();
	let mut res = None;
	for (i, card) in cards.iter().enumerate() {
		if card.name.to_lowercase().starts_with(&s) {
			println!("match!");
			res = if res.is_none() {Some(i)} else {return None};
		}
	}
	res
}

pub fn err(s: &str) -> Option<String> {Some(s.to_string())}

pub fn pick_spirit(g: &mut Game, m: u8) -> Option<String> {
	if g.owned.len() != 2 {
		return err("invalid spirit setup. this is a bug!");
	}
	margin(m);
	println!("{} or {}?", g.owned[0].name, g.owned[1].name);
	let inp = prompt(m).chars().collect::<Vec<_>>();
	let opt = inp.as_slice();
	if let Some(c) = pref_filter(opt, &g.owned) {
		if let Score::Spirit(_, _, suns) = g.owned[c].score {
			g.suns = suns;
		}
		g.owned.remove(1-c);
		None
	}
	else {err("no such card with that prefix!")}
}

// potential inputs:
// 1-3: take a pool
// t/l/r/b/f/w+[yx]: place a tile from held
// \w{3,}: take a card from the shop
// \w{3,}[yx]: place a cube from owned cards
// !\w{3,}/\w{3}!: trash a card from the shop
pub fn ask(g: &mut Game, m: u8) -> Option<String> {
	let arr = prompt(m).chars().collect::<Vec<_>>();
	let s = arr.as_slice();
	match s {
		['q'] => {g.done = true;},
		[c@'1'..='3'] => {
			// pools 
			if !g.held.is_empty() {
				return err("finish placing current hand before taking a new one!");
			}
			if g.grid.done() {
				return err("no more tiles can be taken!");
			}
			g.held = g.pools.swap_remove(*c as usize-'1' as usize);
			g.new_pools();
		},
		[c@('t'|'l'|'r'|'b'|'f'|'w'), xc@'a'..='j', yc@'1'..='9'] => {
			// place tile from held
			let mut tile = None;
			let mut pos = 0;
			for i in 0..g.held.len() {
				if g.held[i].symbol() == *c {
					tile = Some(g.held[i]);
					pos = i;
					break;
				}
			}
			if tile.is_none() {return err("no such tile in hand!")}
			let y = *yc as usize-'1' as usize; let x = *xc as usize-'a' as usize;
			if g.grid.stack(y, x, &tile.unwrap()).is_none() {
				return err("that tile can't be placed there!");
			};
			g.held.remove(pos);
			if g.held.is_empty() {g.cardact = false;}
		},
		['!', p@..]|[p@.., '!'] => {
			// trash from shop			
			if g.cardact {return err("you've already used your card action!");}
			let Some(i) = pref_filter(p, &g.shop) else {
				return err("no such card with that prefix!");
			};
			g.shop.remove(i);
			if let Some(c) = g.deck.pop() {
				g.shop.push(c);
			}
			g.cardact = true;
		},
		[xc@'a'..='j', yc@'1'..='9', p@..]|[p@.., xc@'a'..='j', yc@'1'..='9'] => {
			let Some(i) = pref_filter(p, &g.owned) else {
				return err("no such card with that prefix!");
			};
			let c = g.owned.get_mut(i).unwrap();
			let y = *yc as usize-'1' as usize; let x = *xc as usize-'a' as usize;
			if g.grid.valid_cube(y as isize, x as isize, c) {
				g.grid.hexes[y][x].cube = true;
				match &c.score {
					Score::Cubes(pts) => {g.score += pts[c.cubes];},
					s@Score::Spirit(..) => {g.spirit = Some(s.clone());}
				}
				c.cubes += 1;
				if (*c).done() {
					g.owned.remove(i);
				}
			}
			else {return err("that cube can't be placed there!");}
		},
		_ => {
			// take from shop
			if g.cardact {return err("you've already used your card action!");}
			let Some(i) = pref_filter(s, &g.shop) else {
				return err("no such card with that prefix!");
			};
			if g.owned.len() >= 4 {
				return err("you don't have any more room for cards!");
			}
			g.owned.push(g.shop.remove(i));
			if let Some(c) = g.deck.pop() {
				g.shop.push(c);
			}
			g.cardact = true;
		}
	}
	None
}
