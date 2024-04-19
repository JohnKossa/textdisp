//! Render example where each glyph pixel is output as an ascii character.
use image::open;
use clap::Parser;
mod lib;
use lib::img_to_text;
use lib::img_to_colored_terminal_text;

#[derive(Parser, Debug)]
#[command(about = "Converts an image to ascii art and displays it in the terminal.")]
#[command(version, long_about = None)]
struct Args {
	image_path: String,
	#[arg(short, long)]
	/// Width of the output text in characters
	width: Option<usize>,
	#[arg(short, long)]
	/// Height of the output text in characters
	height: Option<usize>,
	#[arg(short, long)]
	/// Scale mappings to image range instead of absolute range
	normalize: bool,
	#[arg(short, long)]
	/// Invert value scale
	invert: bool,
	#[arg(short, long)]
	/// Display with tagged terminal colors
	colorize: bool
}

fn main() {
	let matches = Args::parse();

	//take in image as png/frame/etc
	let img_path = &matches.image_path;

	//take in width and height (in characters)
	let width: Option<usize> = matches.width;
	let height: Option<usize> = matches.height;

	// Check for normalize flag in arguments
	let normalize: bool = matches.normalize;

	// Check for invert flag in arguments
	let invert: bool = matches.invert;

	//Check for color flag in arguments
	let colorize: bool = matches.colorize;

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
	println!("{}", result);
}
