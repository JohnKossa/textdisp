//! Render example where each glyph pixel is output as an ascii character.
use image::open;
use std::{env};
mod lib;
use lib::img_to_text;

fn main() {
	let args: Vec<String> = env::args().collect();

	//take in image as png/frame/etc
	let img_path = match args.get(1){
		Some(str) => str,
		None => {
			println!("Please specify the path to an image file.");
			return;
		}
	};

	//take in width and height (in characters)
	let width =  match args.get(2) {
		Some(str) => match str.parse::<usize>(){
			Ok(w) => w,
			Err(_) => {
				println!("Please specify a positive integer for width.");
				return;
			}
		}
		None => {
			println!("Please specify the width (in characters) of the output string.");
			return;
		}
	};
	let height = match args.get(3) {
		Some(str) => match str.parse::<usize>(){
			Ok(w) => w,
			Err(_) => {
				println!("Please specify a positive integer for height.");
				return;
			}
		}
		None => {
			println!("Please specify the height (in characters) of the output string.");
			return;
		}
	};
	// Check for normalize flag in arguments
	let normalize: bool = args.iter().find_map(|arg| {
		if arg.starts_with("--normalize=") {
			match &arg["--normalize=".len()..].parse::<bool>() {
				Ok(val) => Some(*val),
				Err(_) => {
					println!("Please specify true or false for normalization.");
					None
				}
			}
		} else {
			None
		}
	}).unwrap_or(false);

	let img = open(img_path).unwrap().into_rgba8();

	let result = img_to_text(img, width, height, normalize);
	println!("{}", result);
}
