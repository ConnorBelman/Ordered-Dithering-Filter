
#[macro_use]
extern crate clap;
extern crate image;

use clap::{Arg, App};
use image::RgbImage;
use image::GenericImageView;

const THRESHOLD_MAP_2X2: [f32; 4] = [
	0., 3.,
	2., 1.
];

const THRESHOLD_MAP_4X4: [f32; 16] = [
	0.,  12., 3.,  15.,
	8.,  4.,  11., 7. ,
	2.,  14., 1.,  13.,
	10., 6.,  9.,  5. 
];

const THRESHOLD_MAP_8X8: [f32; 64]  = [
	0.,  48., 12., 60., 3.,  51., 15., 63.,
	32., 16., 44., 28., 35., 19., 47., 31.,
	8.,  56., 4.,  52., 11., 59., 7.,  55.,
	40., 24., 36., 20., 43., 27., 39., 23.,
	2.,  50., 14., 62., 1.,  49., 13., 61.,
	34., 18., 46., 30., 33., 17., 45., 29.,
	10., 58., 6.,  54., 9.,  57., 5.,  53.,
	42., 26., 38., 22., 41., 25., 37., 21.
];

const THRESHOLD_MAP_4X2: [f32; 8] = [
	0., 4., 2., 6.,
	3., 7., 1., 5.
];

const THRESHOLD_MAP_8X2: [f32; 16] = [
	0., 8.,  4., 12., 2., 10., 6., 14.,
	3., 11., 7., 15., 1., 9.,  5., 13.
];

const THRESHOLD_MAP_8X4: [f32; 32] = [
	0.,  16., 8.,  24., 2.,  18., 10., 26.,
	12., 28., 4.,  20., 14., 30., 6.,  22.,
	3.,  19., 11., 27., 1.,  17., 9.,  25.,
	15., 31., 7.,  23., 13., 29., 5.,  21.
];

const THRESHOLD_MAP_3X3: [f32; 9] = [
	0., 5., 2.,
	3., 8., 7.,
	6., 1., 4.
];

const THRESHOLD_MAP_5X3: [f32; 15] = [
	0.,  12., 7.,  3.,  9. ,
	14., 8.,  1.,  5.,  11.,
	6.,  4.,  10., 13., 2. 
];

const THRESHOLD_MAP_4X1: [f32; 4] = [
	0., 2., 1., 3.
];

const PALETTE:[[u8; 3]; 18] = [
	[0x00,0x00,0x00], [0x00,0x80,0x00], [0x00,0xFF,0x00],
	[0x00,0x00,0xFF], [0x00,0x80,0xFF], [0x00,0xFF,0xFF],
	[0x80,0x00,0x00], [0x80,0x80,0x00], [0x80,0xFF,0x00],
	[0x80,0x00,0xFF], [0x80,0x80,0xFF], [0x80,0xFF,0xFF],
	[0xFF,0x00,0x00], [0xFF,0x80,0x00], [0xFF,0xFF,0x00],
	[0xFF,0x00,0xFF], [0xFF,0x80,0xFF], [0xFF,0xFF,0xFF]
];

const THRESHOLD: [f32; 3] = [255./3., 255./3., 255./2.]; 

fn color_distance(r1: i16, g1: i16, b1: i16, r2: u8, g2: u8, b2: u8) -> f32 {
	let diff_r: i32 = r1 as i32 - r2 as i32;
	let diff_g: i32 = g1 as i32 - g2 as i32;
	let diff_b: i32 = b1 as i32 - b2 as i32;
	let dis_squared: f32 = ((diff_r * diff_r) + (diff_g * diff_g) + (diff_b * diff_b)) as f32;
	dis_squared.sqrt()
}

fn closest_color([r1, g1, b1]: [i16; 3]) -> [u8; 3] {
	let mut best_match: [u8; 3] = [0, 0 ,0];
	let mut best_distance: f32 = std::f32::MAX;
	for i in 0..PALETTE.len() {
		let [r2, g2, b2] = PALETTE[i];
		let distance: f32 = color_distance(r1, g1, b1, r2, g2, b2);
		if distance <= best_distance {
			best_match = [r2, g2, b2];
			best_distance = distance;
		}
	} 
	best_match
}

fn dither(img_file: String, threshold_map: &[f32], length: usize, height: usize, divisor: f32) -> RgbImage {
	let img = image::open(img_file).unwrap();
	let mut output = img.to_rgb();
	let offset: f32 = 0.4;
	for (x, y, pixel) in img.pixels() {
		let r: i16 = pixel[0] as i16 + ((threshold_map[(y as usize % height) * height + (x as usize % length)] / divisor - offset) * THRESHOLD[0]) as i16;
		let g: i16 = pixel[1] as i16 + ((threshold_map[(y as usize % height) * height + (x as usize % length)] / divisor - offset) * THRESHOLD[1]) as i16;
		let b: i16 = pixel[2] as i16 + ((threshold_map[(y as usize % height) * height + (x as usize % length)] / divisor - offset) * THRESHOLD[2]) as i16;
		output.put_pixel(x, y, image::Rgb(closest_color([r, g, b])));
	}
	output
}

fn main() {
	let matches = App::new("Ordered Dither Filter")
        .version(crate_version!())
        .author("Connor Belman")
        .about("Reduces an image to an 18 color palette with ordered dithering")
        .after_help("Matrix sizes: 2x2, 4x4, 8x8, 4x2, 4x1, 8x2, 8x4, 3x3, 5x3")
        .arg(Arg::with_name("input_file")
                .required(true)
                .index(1)
                .help("input file to be filtered"))
        .arg(Arg::with_name("output_file")
        	.required(true)
        	.index(2)
        	.help("path to output file"))
        .arg(Arg::with_name("matrix")
        	.short("m")
        	.long("matrix")
        	.takes_value(true)
        	.help("Bayer Matrix to use [default: 4x4]"))
        .get_matches();

    let output = match matches.value_of("matrix").unwrap_or("4x4") {
    	"2x2" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_2X2, 2, 2, 4. ),
    	"4x4" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_4X4, 4, 4, 16.),
    	"8x8" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_8X8, 8, 8, 64.),
    	"4x2" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_4X2, 4, 2, 8. ),
    	"8x2" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_8X2, 8, 2, 16.),
    	"8x4" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_8X4, 8, 4, 32.),
    	"3x3" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_3X3, 3, 3, 9. ),
    	"5x3" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_5X3, 5, 3, 15.),
    	"4x1" => dither(matches.value_of("input_file").unwrap().to_string(), &THRESHOLD_MAP_4X1, 4, 1, 4. ),
    	_ => panic!("invalid matrix size")
    };
    output.save(matches.value_of("output_file").unwrap()).unwrap();
    println!("Done! Image saved to {}", matches.value_of("output_file").unwrap());
}
