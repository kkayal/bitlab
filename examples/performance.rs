extern crate bitlab;

use bitlab::*;
use std::time::{Instant};

fn main() {
	let a = 0xFFu8;
	let mut _b:u8 = 0;
	let n = 1_000_001;

	let now = Instant::now();
	for _n in 1..n {
		_b = a.get_u8(5, 3).unwrap();
	}

	println!("Duration: {} seconds and {} nanoseconds for {} runs", now.elapsed().as_secs(), now.elapsed().subsec_nanos(), n-1);
}