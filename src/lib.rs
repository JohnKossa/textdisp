use std::cmp::{min,max};
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