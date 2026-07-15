use std::prelude::*;

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
		return format!("\x1b[{}m",
			match self {
				Color::Norm => 0,
				Color::Red => 31,
				Color::Green => 32,
				Color::Yellow => 33,
				Color::Blue => 34,
				Color::Magenta => 35,
				Color::Cyan => 36
			}
		);
	}
}


pub fn use_color() -> bool {true}
pub fn color_red() -> String {String::from(if use_color() {"\x1b[31m"} else {""})}
pub fn color_green() -> String {String::from(if use_color() {"\x1b[32m"} else {""})}
pub fn color_yellow() -> String {String::from(if use_color() {"\x1b[33m"} else {""})}
pub fn color_blue() -> String {String::from(if use_color() {"\x1b[34m"} else {""})}
pub fn color_magenta() -> String {String::from(if use_color() {"\x1b[35m"} else {""})}
pub fn color_cyan() -> String {String::from(if use_color() {"\x1b[36m"} else {""})}
pub fn color_reset() -> String {String::from(if use_color() {"\x1b[0m"} else {""})}
pub fn color_under() -> String {String::from(if use_color() {"\x1b[4m"} else {""})}

//impl Hex {
//	pub fn draw(&self) {
//		print!("{}{}", match self {
//			Hex::Empty => ".".to_string(),
//			Hex::Wood => color_magenta()+"w",
//			Hex::TwoWood => color_magenta()+"W",
//			Hex::Leaf => color_green()+"l",
//			Hex::Tree => color_green()+"L",
//			Hex::TwoTree => color_green()+"T",
//			Hex::Rock => color_cyan()+"r",
//			Hex::TwoRock => color_cyan()+"R",
//			Hex::ThreeRock => color_cyan()+"M",
//			Hex::Brick => color_red()+"b",
//			Hex::Building => color_red()+"B",
//			Hex::Field => color_yellow()+"f",
//			Hex::Water => color_blue()+"~"
//		}, color_reset());
//	}
//}
