extern crate bitlab;

use bitlab::*;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	// Thanks to http://www.matthewflickinger.com/lab/whatsinagif/bits_and_bytes.asp
	let file_name = "examples/sample_1.gif";
	match read_file(file_name) {
		Ok(vec) => {
			match vec.get_u8(10, 1, 3) {
				Ok(color_resolution) => {
					println!("The packed byte is at byte offset 10 = 0x{:X} = {:b}", vec[10], vec[10]);
					println!("The color resolution starts at \
						bit offset 1 and is 3 bits long: (binary)");
					println!("binary  : {:03b}", color_resolution);
					println!("decimal : {}", color_resolution);
				},
				Err(e) => panic!("Failed to read the color resolution: {:?}", e)
			}
		},
		Err(e) => panic!("Unable to open {} {:?}", file_name, e)
	}
}

fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
	let mut f = File::open(path)?;	// If that fails return early and pass the error message
	let metadata = f.metadata()?;	// The ? is the same as the try!()
	let size = metadata.len() as usize;
	let mut v: Vec<u8> = Vec::new();
	v.resize(size, 0);
	f.read_exact(&mut v)?;
	Ok(v)
}
