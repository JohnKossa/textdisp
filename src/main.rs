//! Render example where each glyph pixel is output as an ascii character.
use std::cmp::min;
use image::{open, DynamicImage, RgbaImage};
use std::{env};

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
	let normalize = match args.get(4) {
		Some(str) => match str.parse::<bool>(){
			Ok(w) => w,
			Err(_) => {
				println!("Please specify true or false for normalization.");
				return;
			}
		}
		None => false
	};

	let img = open(img_path).unwrap().into_rgba8();

	let result = img_to_text(img, width, height, normalize);
	println!("{}", result);
}

pub fn img_to_text(image: RgbaImage, width: usize, height: usize, normalize: bool) -> String{
	let gray_img =  DynamicImage::ImageRgba8(image).into_luma8();
	let img_width = gray_img.width();
	let img_height = gray_img.height();

	//get chars per unit width
	//get chars per unit height
	let x_scale_f = img_width as f32 / width as f32; //pixels per char
	let y_scale_f = img_height as f32 / height as f32 ; //pixels per char

	let mut value_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut counts_buffer= vec![vec![0; height]; width];


	for x in 0..img_width{
		for y in 0..img_height{
			let x_coord = min((x as f32 /x_scale_f) as usize,width-1);
			let y_coord = min((y as f32 /y_scale_f) as usize, height-1);
			let luma = gray_img.get_pixel(x, y);
			value_buffer[x_coord as usize][y_coord as usize] += luma[0] as u128;
			counts_buffer[x_coord as usize][y_coord as usize] += 1;
		}
	}
	let mut avg_value_buffer = vec![vec![0.0; height]; width];

	for x in 0..width {
		for y in 0..height {
			avg_value_buffer[x][y] = value_buffer[x][y] as f64 / counts_buffer [x][y] as f64
		}
	}

	let mut max_value = 255.0;
	let mut min_value = 0.0;
	if normalize {
		min_value = avg_value_buffer
			.iter()
			.flatten()
			.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
			.unwrap()
			.clone();
		max_value = avg_value_buffer
			.iter()
			.flatten()
			.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
			.unwrap()
			.clone();
	}

	let standard_ascii_value_map= b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
	let trimmed_ascii_value_map = b"@%#x+=:-. ";

	let value_map_length = trimmed_ascii_value_map.len();
	let mut out_buffer= vec![vec![' '; height]; width];
	for x in 0..width {
		for y in 0..height {
			let value_pct = (avg_value_buffer[x][y] - min_value)/max_value;
			//println!("{}", value_pct);
			let idx = min((value_pct * (value_map_length as f64)) as usize,value_map_length-1);
			out_buffer[x][y] = trimmed_ascii_value_map[idx] as char;
		}
	}

	let mut result=String::from("");
	for y in 0..height{
		for x in 0..width {
			result.push(out_buffer[x][y]);
		}
		result.push('\n')
	}
	result
}