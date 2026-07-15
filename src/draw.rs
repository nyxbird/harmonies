use std::prelude::*;
use crate::{grid::*, util::*};

impl Grid {
	//pub fn draw(&self) {
	//	for i in 0..self.h {
	//		print!("  ");
	//		for s in 0..self.w-self.hexes[i].len() {
	//			print!(" ");
	//		}
	//		print!("{} ", i);
	//		for &hex in self.hexes[i].iter() {
	//			print!("{} ", hex.show());
	//		}
	//		if i > self.h/2 {
	//			print!("{}", self.w-i+self.h/2);
	//		}
	//		println!();
	//	}
	//	for s in 0..(self.h/2+5) {print!(" ");}
	//	for i in 0..self.w-self.h/2 {
	//		print!("{} ", i);
	//	}
	//	println!();
	//}

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
			lines[0].push((b'a'+i as u8) as char);
			lines[0].push(' ');
		}
		for i in 1..self.h+1 {
			lines[i] += &format!("{} ", i);
			if i%2 == 0 {lines[i].push(' ');}
			for (j, &hex) in self.hexes[i-1].iter().enumerate() {
				if(self.cubes[i-1][j]) {lines[i] += &color_under();}
				lines[i] += &hex.show();
				lines[i].push(' ');
			}
		}
		lines
	}
}
