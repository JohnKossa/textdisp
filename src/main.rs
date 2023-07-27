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
	let width: Option<usize> = args.iter().find_map(|arg| {
		if arg.starts_with("--width=") {
			match &arg["--width=".len()..].parse::<usize>() {
				Ok(val) => Some(*val),
				Err(_) => {
					println!("Please specify a positive integer for width.");
					None
				}
			}
		} else {
			None
		}
	});
	let height: Option<usize> = args.iter().find_map(|arg| {
		if arg.starts_with("--height=") {
			match &arg["--height=".len()..].parse::<usize>() {
				Ok(val) => Some(*val),
				Err(_) => {
					println!("Please specify a positive integer for height.");
					None
				}
			}
		} else {
			None
		}
	});
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

	// Check for invert flag in arguments
	let invert: bool = args.iter().find_map(|arg| {
		if arg.starts_with("--invert=") {
			match &arg["--invert=".len()..].parse::<bool>() {
				Ok(val) => Some(*val),
				Err(_) => {
					println!("Please specify true or false for value inversion.");
					None
				}
			}
		} else {
			None
		}
	}).unwrap_or(false);

	let img = open(img_path).unwrap().into_rgba8();

	let result = img_to_text(img, width, height, normalize, invert);
	println!("{}", result);
}
