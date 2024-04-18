use std::cmp::{min};
use std::collections::HashMap;
use image::{DynamicImage, RgbaImage};

pub fn img_to_text(image: RgbaImage, requested_width: Option<usize>, requested_height: Option<usize>, normalize: bool, invert_value: bool) -> String{
	let gray_img =  DynamicImage::ImageRgba8(image).into_luma8();
	let img_width = gray_img.width();
	let img_height = gray_img.height();

	let (width, height) = match (requested_width, requested_height){
		(Some(w), Some(h)) => (w,h),
		(Some(w), None) =>{
			//ideal ratio is width being double height, since characters are usually 1x2
			//look at image to get missing value
			let h = w * img_height as usize/ img_width as usize;
			(w,h)
		},
		(None, Some(h)) => {
			//ideal ratio is width being double height, since characters are usually 1x2
			//look at image to get missing value
			let w = 2 * h * img_width as usize / img_height as usize;
			(w,h)
		}
		(None, None) => (80,40)
	};

	//get chars per unit width
	//get chars per unit height
	let x_scale_f = img_width as f32 / width as f32; //pixels per char
	let y_scale_f = img_height as f32 / height as f32 ; //pixels per char

	let mut value_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut counts_buffer= vec![vec![0; height]; width];


	//for each pixel in the image, get the luma value and add it to the corresponding cell in the buffer
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

	let max_value = match normalize {
		false => 255.0,
		true => avg_value_buffer
					.iter()
					.flatten()
					.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let min_value= match normalize {
		false => 0.0,
		true => avg_value_buffer
					.iter()
					.flatten()
					.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};

	let standard_ascii_value_map= b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
	let trimmed_ascii_value_map = b"@%#x+=:-. ";

	let value_map_length = trimmed_ascii_value_map.len();
	let mut out_buffer= vec![vec![' '; height]; width];
	for x in 0..width {
		for y in 0..height {
			let value_pct = (avg_value_buffer[x][y] - min_value)/max_value;
			//println!("{}", value_pct);
			//let idx = min((value_pct * (value_map_length as f64)) as usize,value_map_length-1);
			let idx = match invert_value{
				false => min((value_pct * (value_map_length as f64)) as usize,value_map_length-1),
				true =>  min(((1.0 - value_pct) * (value_map_length as f64)) as usize, value_map_length-1)
			};
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

pub fn img_to_colored_terminal_text(image: RgbaImage, requested_width: Option<usize>, requested_height: Option<usize>, normalize: bool, invert_value: bool) -> String{
	let color_img = DynamicImage::from(image.clone()).into_rgba8();
	//let color_img = DynamicImage::ImageRgba8(image.copy()).into_rgba8();
	let gray_img =  DynamicImage::ImageRgba8(image).into_luma8();
	//let gray_img =  DynamicImage::ImageRgba8(image).into_luma8();

	let img_width = color_img.width();
	let img_height = color_img.height();

	let (width, height) = match (requested_width, requested_height){
		(Some(w), Some(h)) => (w,h),
		(Some(w), None) =>{
			//ideal ratio is width being double height, since characters are usually 1x2
			//look at image to get missing value
			let h = w * img_height as usize/ img_width as usize;
			(w,h)
		},
		(None, Some(h)) => {
			//ideal ratio is width being double height, since characters are usually 1x2
			//look at image to get missing value
			let w = 2 * h * img_width as usize / img_height as usize;
			(w,h)
		}
		(None, None) => (80,40)
	};

	//get chars per unit width
	//get chars per unit height
	let x_scale_f = img_width as f32 / width as f32; //pixels per char
	let y_scale_f = img_height as f32 / height as f32 ; //pixels per char

	let mut value_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut red_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut green_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut blue_buffer:Vec<Vec<u128>>= vec![vec![0; height]; width];
	let mut counts_buffer= vec![vec![0; height]; width];

	for x in 0..img_width{
		for y in 0..img_height{
			let x_coord = min((x as f32 /x_scale_f) as usize,width-1);
			let y_coord = min((y as f32 /y_scale_f) as usize, height-1);
			let pixel = color_img.get_pixel(x, y);
			value_buffer[x_coord as usize][y_coord as usize] += gray_img.get_pixel(x, y)[0] as u128;
			red_buffer[x_coord as usize][y_coord as usize] += pixel[0] as u128;
			green_buffer[x_coord as usize][y_coord as usize] += pixel[1] as u128;
			blue_buffer[x_coord as usize][y_coord as usize] += pixel[2] as u128;
			counts_buffer[x_coord as usize][y_coord as usize] += 1;
		}
	}

	let mut avg_value_buffer = vec![vec![0.0; height]; width];
	let mut avg_red_buffer = vec![vec![0.0; height]; width];
	let mut avg_green_buffer = vec![vec![0.0; height]; width];
	let mut avg_blue_buffer = vec![vec![0.0; height]; width];

	for x in 0..width {
		for y in 0..height {
			avg_value_buffer[x][y] = value_buffer[x][y] as f64 / counts_buffer[x][y] as f64;
			avg_red_buffer[x][y] = red_buffer[x][y] as f64 / counts_buffer[x][y] as f64;
			avg_green_buffer[x][y] = green_buffer[x][y] as f64 / counts_buffer[x][y] as f64;
			avg_blue_buffer[x][y] = blue_buffer[x][y] as f64 / counts_buffer[x][y] as f64;
		}
	}

	let max_value = match normalize {
		false => 255.0,
		true => avg_value_buffer
					.iter()
					.flatten()
					.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};

	let min_value = match normalize {
		false => 0.0,
		true => avg_value_buffer
					.iter()
					.flatten()
					.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};

	let max_red = match normalize {
		false => 255.0,
		true => avg_red_buffer
					.iter()
					.flatten()
					.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let min_red = match normalize {
		false => 0.0,
		true => avg_red_buffer
					.iter()
					.flatten()
					.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let max_green = match normalize {
		false => 255.0,
		true => avg_green_buffer
					.iter()
					.flatten()
					.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let min_green = match normalize {
		false => 0.0,
		true => avg_green_buffer
					.iter()
					.flatten()
					.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let max_blue = match normalize {
		false => 255.0,
		true => avg_blue_buffer
					.iter()
					.flatten()
					.max_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};
	let min_blue = match normalize {
		false => 0.0,
		true => avg_blue_buffer
					.iter()
					.flatten()
					.min_by(|&&a, &b| a.partial_cmp(b).unwrap())
					.unwrap()
					.clone()
	};

	let standard_ascii_value_map= b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
	let trimmed_ascii_value_map = b"@%#x+=:-. ";
	let mut color_code_map: HashMap<u8, (f64, f64, f64)> = HashMap::new();
	color_code_map.insert(30, (0.0, 0.0, 0.0));
	color_code_map.insert(31, (1.0, 0.0, 0.0));
	color_code_map.insert(32, (0.0, 1.0, 0.0));
	color_code_map.insert(33, (1.0, 1.0, 0.0));
	color_code_map.insert(34, (0.0, 0.0, 1.0));
	color_code_map.insert(35, (1.0, 0.0, 1.0));
	color_code_map.insert(36, (0.0, 1.0, 1.0));
	color_code_map.insert(37, (1.0, 1.0, 1.0));
	let reset_code = "\x1b[0m";

	//Create a map of all terminal color strings
	//let color_map = vec!["\x1b[38;2;255;0;0m", "\x1b[38;2;0;255;0m", "\x1b[38;2;0;0;255m", "\x1b[38;2;255;255;0m", "\x1b[38;2;0;255;255m", "\x1b[38;2;255;0;255m"];

	let value_map_length = trimmed_ascii_value_map.len();
	let mut out_buffer= vec![vec![" ".to_string(); height]; width];
	for x in 0..width {
		for y in 0..height {
			let red_pct = (avg_red_buffer[x][y] - min_red)/max_red;
			let green_pct = (avg_green_buffer[x][y] - min_green)/max_green;
			let blue_pct = (avg_blue_buffer[x][y] - min_blue)/max_blue;
			let value_pct = (avg_value_buffer[x][y] - min_value)/max_value;
			//find the closest color code
			//sort color codes by closest
			let closest_color_code = color_code_map.iter()
				.min_by(|(_, (r, g, b)), (_, (r2, g2, b2))| {
					let dr =  r - red_pct;
					let dg = g - green_pct;
					let db = b - blue_pct;
					let dr2 = r2 - red_pct;
					let dg2 = g2 - green_pct;
					let db2 = b2 - blue_pct;
					let comp1 = dr * dr + dg * dg + db * db;
					let comp2 = dr2 * dr2 + dg2 * dg2 + db2 * db2;
					comp1.partial_cmp(&comp2).unwrap()
				})
				.unwrap().0;

			let idx = match invert_value{
				false => min((value_pct * (value_map_length as f64)) as usize,value_map_length-1),
				true =>  min(((1.0 - value_pct) * (value_map_length as f64)) as usize, value_map_length-1)
			};
			let char_mapped = trimmed_ascii_value_map[idx] as char;
			let color_code = format!("\x1b[{}m", closest_color_code);
			out_buffer[x][y] = format!("{}{}", color_code, char_mapped);
		}
	}
	let mut result=String::from("");
	for y in 0..height{
		for x in 0..width {
			result = result + out_buffer[x][y].as_str();
		}
		result.push('\n')
	}
	result = result+reset_code;
	result
	// out_buffer
	// 	.iter()
	// 	.map(|row| row.join(""))
	// 	.collect::<Vec<String>>().join("\n")+reset_code
}