pub enum Color {
	Norm,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan
}

use Color::*;

impl Color {
	pub fn ansi(&self) -> String {
		if !use_color() {return "".to_string();}
		format!("\x1b[{}m",
			match self {
				Color::Norm => 0,
				Color::Red => 31,
				Color::Green => 32,
				Color::Yellow => 33,
				Color::Blue => 34,
				Color::Magenta => 35,
				Color::Cyan => 36
			}
		)
	}

	pub fn wrap(&self, s: &str) -> String {
		format!("{}{}{}", self.ansi(), s, Norm.ansi())
	}
}

pub fn norm_len(s: &str) -> usize {
	let mut len = 0;
	let mut esc = false;
	for c in s.chars() {
		if c == '\x1b' {esc = true;}
		if !esc {len += 1;}
		if c == 'm' {esc = false;}
	}
	len
}

pub fn use_color() -> bool {true}
pub fn color_reset() -> String {(if use_color() {"\x1b[0m"} else {""}).to_string()}
pub fn clear() -> String {"\x1b[3J\x1b[H\x1b[2J".to_string()}
