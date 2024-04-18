//! Render example where each glyph pixel is output as an ascii character.
use image::open;
use std::{env};
mod lib;
use lib::img_to_text;
use lib::img_to_colored_terminal_text;

fn main() {
	let args: Vec<String> = env::args().collect();

	//if there are no parameters, show usage text
	if args.len() == 1 {
		println!("Usage: textdisp <image_path> [--width=<width>] [--height=<height>] [--normalize=<true/false>] [--invert=<true/false>]");
		return;
	}

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

	//Check for color flag in arguments
	let colorize: bool = args.iter().find_map(|arg| {
		if arg.starts_with("--colorize=") {
			match &arg["--colorize=".len()..].parse::<bool>() {
				Ok(val) => Some(*val),
				Err(_) => {
					println!("Please specify true or false for colorization.");
					None
				}
			}
		} else {
			None
		}
	}).unwrap_or(false);

	let img = match open(img_path){
		Ok(image_result) => image_result.into_rgba8(),
		Err(_) => {
			println!("Please specify a valid path to an image file.");
			return;
		}
	};

	let result = match colorize {
		true => img_to_colored_terminal_text(img, width, height, normalize, invert),
		false => img_to_text(img, width, height, normalize, invert)
	};
	//let result = img_to_text(img, width, height, normalize, invert);
	println!("{}", result);
}
