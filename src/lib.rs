//![![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)
//!
//!# Objective:
//!
//!To extract a range of bits from a binary data source
//!
//!# Status
//!
//!Experimental
//!
//!# Documentation
//!
//!This crate is published at [crates.io](https://crates.io/crates/bitlab). The detailed documentation is available at [docs.rs/bitlab](https://docs.rs/bitlab/)
//!
//!# Version
//!
//!0.4.3
//!
//!# Usage
//!
//!1. In your Cargo.toml file, add `bitlab = "0.4"` under `[dependencies]`
//!2. In your source file, add `extern crate bitlab` and `use bitlab::*;`
//!
//!## Example 1: 
//!
//!Start at bit offset 1, extract 3 bits and interpret the result as u8
//!
//!```rust
//!use bitlab::*;
//!let a: i8 = -33; // = 0b1101_1111;
//!let b = a.get_u8(1, 3).unwrap();  // 1 --> 101 <-- 1111
//!//                                         = 5
//!assert_eq!(b, 5);
//!```
//!
//!## Example 2: 
//!
//!The data source is a vector of u8 types. We want to go to byte offset 1, 
//!bit offset 7 and starting from there extract 3 bits as an u16
//!
//!```rust
//!use bitlab::*;
//!let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
//!let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110_000  --> 1_01 <-- 10_1100
//!//                                                                         = 5
//!assert_eq!(bar.unwrap(), 5);
//!```
//!
//!## Example 3:
//!
//!There is a very simple application in the examples directory,
//!which extracts the color resolution from a real gif file.
//!To run it enter the following in the command line
//!
//!```cli
//!cargo run --release --example gif
//!```
//!
//!# MIT Licence
//!
//! Copyright <2017, Kağan Kayal>
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![warn(missing_docs)]

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
	html_favicon_url = "https://www.rust-lang.org/favicon.ico",
	html_root_url = "https://doc.rust-lang.org/")]

static OUT_OF_RANGE_MSG: &str = "Out of range";

/// Defines a number of functions, which extract a range of bits from
/// primitive numeric types (u8, u16, u32 and u64, i8, i16, i32 and i64) and return
/// the result as one of the following types (u8, u16, u32 and u64, i8, i16, i32 and i64)
/// E.g. the a.get_u8(5,3) function extracts the bits 5,6 and 7 of
/// the variable a and returns the result as a u8 variable
pub trait ExtractBitsFromIntegralTypes {
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u8**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 8)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u16**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 16)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u32**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 32)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u64**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 64)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i8**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 8)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i16**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 16)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i32**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 32)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i64**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source (start + length > 64)
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str>;
}

impl ExtractBitsFromIntegralTypes for u8 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 8 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err(OUT_OF_RANGE_MSG);
		}
		// Note that I did think about using mem::size_of::<T>() instead of
		// the literal 8 above. This would also deliver the 8 at compile time and
		// I could get a step closer to implement this function in a more
		// generic way. But it doesn't help, because the type conversion below as
		// let mut copy = *self as T; doesn't compile for good reasons.
		// I also tried to implement a macro, but that I failed to get it right
		// So, I decided to stick to a simple trait without geneics and implement
		// the same thing for many type combinations... It is more work and
		// more risky to make copy & paste errors. True, but the first point is
		// just my problem and the second can be ironed out by extensive unit testing.

		// Don't touch the original
		let mut copy = self as i8;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 8 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		Ok(self.get_u8(start, length)? as u16)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		Ok(self.get_i8(start, length)? as i16)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		Ok(self.get_u8(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		Ok(self.get_i8(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		Ok(self.get_u8(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		Ok(self.get_i8(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i8 {
	// TODO: Check if the type conversions below have an impact to
	// run-time performance or if it is better to optimise
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		(self as u8).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		(self as u8).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		(self as u8).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		(self as u8).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		(self as u8).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		(self as u8).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		(self as u8).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		(self as u8).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u16 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a u8");
		}

		// Return the result
		Ok(self.get_u16(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a i8");
		}

		// Return the result
		Ok(self.get_i16(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 16 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self as i16;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 16 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		Ok(self.get_u16(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		Ok(self.get_i16(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		Ok(self.get_u16(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		Ok(self.get_i16(start, length)? as i64)
	}
}


impl ExtractBitsFromIntegralTypes for i16 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		(self as u16).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		(self as u16).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		(self as u16).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		(self as u16).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		(self as u16).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		(self as u16).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		(self as u16).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		(self as u16).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u32 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a u8");
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a i8");
		}

		// Return the result
		Ok(self.get_i32(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		if length > 16 {
			return Err("The length parameter is too big for a u16");
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		if length > 16 {
			return Err("The length parameter is too big for a i16");
		}

		// Return the result
		Ok(self.get_u32(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 32 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self as i32;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 32 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		Ok(self.get_u32(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		Ok(self.get_i32(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i32 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		(self as u32).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		(self as u32).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		(self as u32).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		(self as u32).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		(self as u32).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		(self as u32).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		(self as u32).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		(self as u32).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u64 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a u8");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		if length > 8 {
			return Err("The length parameter is too big for a i8");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		if length > 16 {
			return Err("The length parameter is too big for a u16");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		if length > 16 {
			return Err("The length parameter is too big for a i16");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		if length > 32 {
			return Err("The length parameter is too big for a u32");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u32)
	}

	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		if length > 32 {
			return Err("The length parameter is too big for a i32");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i32)
	}

	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 64 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err(OUT_OF_RANGE_MSG);
		}

		// Don't touch the original
		let mut copy = self as i64;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 64 - length;

		// Return the result
		Ok(copy)
	}
}

impl ExtractBitsFromIntegralTypes for i64 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, &'static str> {
		(self as u64).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, &'static str> {
		(self as u64).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, &'static str> {
		(self as u64).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, &'static str> {
		(self as u64).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, &'static str> {
		(self as u64).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, &'static str> {
		(self as u64).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, &'static str> {
		(self as u64).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, &'static str> {
		(self as u64).get_i64(start, length)
	}
}

/// Defines a number of functions, which extract a range of bits from a Vec<u8>
/// There is one function for each variable type to be returned
/// **Important:** the contents of the vectored are assumed to be in **big endian** (network) order
pub trait ExtractBitsFromVecU8 {
	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 8 bit unsigned integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 8
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_u8(&self, byte_offset: usize, start: usize, length: usize) -> Result<u8, &'static str>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 8 bit integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 8
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_i8(&self, byte_offset: usize, start: usize, length: usize) -> Result<i8, &'static str>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 16 bit unsigned integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 16
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_u16(&self, byte_offset: usize, start: usize, length: usize) -> Result<u16, &'static str>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 16 bit integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 16
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_i16(&self, byte_offset: usize, start: usize, length: usize) -> Result<i16, &'static str>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 32 bit unsigned integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 32
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_u32(&self, byte_offset: usize, start: usize, length: usize) -> Result<u32, &'static str>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 32 bit integer or an error message.
	///
	/// On success, the Result contains the desired value 
	///
	/// On error, the Result contains an error message. This may happen if:
	///
	/// - bit_offset > 7
	/// - length > 32
	/// - byte_offset * 8 + bit_offset + length > vector (source data) size in bits
	///
	/// Parameters:
	///
	/// - **byte_offset** (usize) the number of bytes to skip in source
	/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (usize) the number of bits to be extracted.
	fn get_i32(&self, byte_offset: usize, start: usize, length: usize) -> Result<i32, &'static str>;
}

impl ExtractBitsFromVecU8 for Vec<u8> {
	fn get_u8(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u8, &'static str> {
		if length <= 8 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					let mut copy: u8 = self[byte_offset];
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = u8::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset;
					// Second, push it all to the right end
					copy >>= 8 - length;
					return Ok(copy);
				} else { // The range of bits spans over 2 bytes (not more)
					// Copy the first byte
					let copy1: u8 = self[byte_offset];

					// Copy that into a bigger variable type
					let mut copy1_as_u16: u16 = copy1 as u16;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_u16 <<= 8;

					// Now copy the second bytes
					let copy2: u8 = self[byte_offset + 1];

					// Logical OR these two to get the original 2 bytes
					let mut result = copy1_as_u16 | (copy2 as u16);

					// From now on, process like the normal case above
					result <<= bit_offset;
					result >>= 16 - length;
					return Ok(result as u8);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}

	fn get_i8(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<i8, &'static str> {
		if length <= 8 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					let mut copy: i8 = self[byte_offset] as i8;
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = i8::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset;
					// Second, push it all to the right end
					copy >>= 8 - length;
					return Ok(copy);
				} else { // The range of bits spans over 2 bytes (not more)
					// Copy the first byte
					let copy1: i8 = self[byte_offset] as i8;

					// Copy that into a bigger variable type
					let mut copy1_as_i16: i16 = copy1 as i16;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_i16 <<= 8;

					// Now copy the second bytes
					let copy2: i8 = self[byte_offset + 1] as i8;

					// Logical OR these two to get the original 2 bytes
					let mut result = copy1_as_i16 | (copy2 as i16);

					// From now on, process like the normal case above
					result <<= bit_offset;
					result >>= 16 - length;
					return Ok(result as i8);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}

	fn get_u16(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u16, &'static str> {
		if length <= 16 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset] as i8;

					// Expand to u16
					let mut copy2 = copy1 as u16;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 8 + bit_offset;

					// Second, push it all to the right end
					copy2 >>= 16 - length;

					return Ok(copy2);
				} else if bit_offset + length <= 16 {
					let mut copy1 = self[byte_offset] as u16;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset + 1] as u16;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset;

					// Second, push it all to the right end
					copy3 >>= 16 - length;

					return Ok(copy3);
				} else { // The range of bits spans over 3 bytes (not more)
					let mut copy1 = self[byte_offset] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset + 1] as u32;
					copy2 <<= 8;

					let copy3 = self[byte_offset + 2] as u32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as u16);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}

	fn get_i16(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<i16, &'static str> {
		if length <= 16 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset] as i8;

					// Expand to i16
					let mut copy2 = copy1 as i16;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 8 + bit_offset;

					// Second, push it all to the right end
					copy2 >>= 16 - length;

					return Ok(copy2);
				} else if bit_offset + length <= 16 {
					let mut copy1 = self[byte_offset] as i16;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset + 1] as i16;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset;

					// Second, push it all to the right end
					copy3 >>= 16 - length;

					return Ok(copy3);
				} else { // The range of bits spans over 3 bytes (not more)
					let mut copy1 = self[byte_offset] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset + 1] as i32;
					copy2 <<= 8;

					let copy3 = self[byte_offset + 2] as i32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as i16);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}

	fn get_u32(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u32, &'static str> {
		if length <= 32 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset] as u8;

					// Expand to u32
					let mut copy2 = copy1 as u32;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 24 + bit_offset;

					// Second, push it all to the right end
					copy2 >>= 32 - length;

					return Ok(copy2);
				} else if bit_offset + length <= 16 {
					let mut copy1 = self[byte_offset] as u32;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset + 1] as u32;
					// copy2 <<= 0;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset + 16;

					// Second, push it all to the right end
					copy3 >>= 32 - length;

					return Ok(copy3);
				} else if bit_offset + length <= 24 {
					let mut copy1 = self[byte_offset] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset + 1] as u32;
					copy2 <<= 8;

					let copy3 = self[byte_offset + 2] as u32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as u32);
				} else if bit_offset + length <= 32 {
					let mut copy1 = self[byte_offset] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 24;

					let mut copy2 = self[byte_offset + 1] as u32;
					copy2 <<= 16;

					let mut copy3 = self[byte_offset + 2] as u32;
					copy3 <<= 8;

					let copy4 = self[byte_offset + 3] as u32;
					// copy4 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy5 = copy1 | copy2 | copy3 | copy4;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy5 <<= bit_offset;

					// Second, push it all to the right end
					copy5 >>= 32 - length;

					return Ok(copy5 as u32);
				} else {
					let mut copy1 = self[byte_offset] as u64;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 32;

					let mut copy2 = self[byte_offset + 1] as u64;
					copy2 <<= 24;

					let mut copy3 = self[byte_offset + 2] as u64;
					copy3 <<= 16;

					let mut copy4 = self[byte_offset + 3] as u64;
					copy4 <<= 8;

					let copy5 = self[byte_offset + 4] as u64;
					// copy5 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy6 = copy1 | copy2 | copy3 | copy4 | copy5;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy6 <<= bit_offset + 32;

					// Second, push it all to the right end
					copy6 >>= 64 - length;

					return Ok(copy6 as u32);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}

	fn get_i32(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<i32, &'static str> {
		if length <= 32 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset] as i8;

					// Expand to i32
					let mut copy2 = copy1 as i32;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 24 + bit_offset;

					// Second, push it all to the right end
					copy2 >>= 32 - length;

					return Ok(copy2);
				} else if bit_offset + length <= 16 {
					let mut copy1 = self[byte_offset] as i32;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset + 1] as i32;
					// copy2 <<= 0;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset + 16;

					// Second, push it all to the right end
					copy3 >>= 32 - length;

					return Ok(copy3);
				} else if bit_offset + length <= 24 {
					let mut copy1 = self[byte_offset] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset + 1] as i32;
					copy2 <<= 8;

					let copy3 = self[byte_offset + 2] as i32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as i32);
				} else if bit_offset + length <= 32 {
					let mut copy1 = self[byte_offset] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 24;

					let mut copy2 = self[byte_offset + 1] as i32;
					copy2 <<= 16;

					let mut copy3 = self[byte_offset + 2] as i32;
					copy3 <<= 8;

					let copy4 = self[byte_offset + 3] as i32;
					// copy4 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy5 = copy1 | copy2 | copy3 | copy4;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy5 <<= bit_offset;

					// Second, push it all to the right end
					copy5 >>= 32 - length;

					return Ok(copy5 as i32);
				} else {
					let mut copy1 = self[byte_offset] as i64;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 32;

					let mut copy2 = self[byte_offset + 1] as i64;
					copy2 <<= 24;

					let mut copy3 = self[byte_offset + 2] as i64;
					copy3 <<= 16;

					let mut copy4 = self[byte_offset + 3] as i64;
					copy4 <<= 8;

					let copy5 = self[byte_offset + 4] as i64;
					// copy5 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy6 = copy1 | copy2 | copy3 | copy4 | copy5;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy6 <<= bit_offset + 32;

					// Second, push it all to the right end
					copy6 >>= 64 - length;

					return Ok(copy6 as i32);
				}
			} else {
				return Err(OUT_OF_RANGE_MSG)
			}
		} else {
			return Err(OUT_OF_RANGE_MSG)
		}
	}
}

/////////////////////////////////////////////////////////////////////
//                                                                 //
//                          UNIT TESTS                             //
//                                                                 //
/////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn range_checks_for_integrals() {
		//
		// Range checks for u8 as source
		//
		let a: u8 = 0x05;

		// Start is OK, Length is OK, but the sum is > 8
		match a.get_u8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		//
		// Range checks for u16 as source
		//
		let a: u16 = 0x05AA;
		match a.get_u8(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(0, 17) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// Start & Length would be OK for the output, but not for the source
		match a.get_u8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_i8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		//
		// Range checks for u32 as source
		//
		let a: u32 = 0x05AAAAAA;
		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u16"),
		}

		match a.get_u32(20, 30) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i16"),
		}

		match a.get_i32(20, 30) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		//
		// Range checks for u64 as source
		//
		let a: u64 = 0x05AAAAAA00000000;
		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u16"),
		}

		match a.get_u32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u32"),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i16"),
		}

		match a.get_i32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i32"),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		//
		// Range checks for i8 as source
		//
		let a: i8 = 0x05;

		// Start is OK, Length is OK, but the sum is > 8
		match a.get_u8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		//
		// Range checks for i16 as source
		//
		let a: i16 = 0x05AA;
		match a.get_u8(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(0, 17) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u16(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// Start & Length would be OK for the output, but not for the source
		match a.get_u8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_i8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		//
		// Range checks for i32 as source
		//
		let a: i32 = 0x05AAAAAA;
		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u16"),
		}

		match a.get_u32(20, 30) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i16"),
		}

		match a.get_i32(20, 30) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		//
		// Range checks for i64 as source
		//
		let a: i64 = 0x05AAAAAA00000000;
		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u16"),
		}

		match a.get_u32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u32"),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_u64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i16"),
		}

		match a.get_i32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a i32"),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		match a.get_i64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}
	}

	#[test]
	fn range_checks_for_vec_u8() {
		//
		// Range checking
		//

		let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"

		// The byte offset has to be < sizeof(vector in bytes)
		match v.get_u8(5, 2, 3) {
			Ok(_) => panic!("The range check failed to detect invalid byte offset"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// The bit offset has to be < 8
		match v.get_u8(1, 8, 10) {
			Ok(_) => panic!("The range check failed to detect invalid bit offset"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// A u8 cannot have 12 bits
		match v.get_u8(1, 5, 12) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// Even if all three parametrs are individually within their range,
		// the combination might leak outside the vector
		match v.get_u8(4, 7, 5) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// A u16 cannot have 17 bits
		match v.get_u16(1, 5, 17) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}

		// Even if all three parametrs are individually within their range,
		// the combination might leak outside the vector
		match v.get_u16(4, 7, 10) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, OUT_OF_RANGE_MSG),
		}
	}

	#[test]
	fn source_must_not_change() {
		// Actually, strictly speaking, we don't need the asserts below.
		// The variable bindings below are not mutable, so
		// the compiler would not compile this file in the first place, if
		// there was a problem with that.
		// Still let's keep them in the unit tetst for the better understanding.

		let a: u8 = 0x05;
		let _b = a.get_u16(3, 4).unwrap();
		assert_eq!(a, 0x05, "The source has changed!");

		let a: u16 = 0x05AA;
		let _b = a.get_u16(5, 3).unwrap();
		assert_eq!(a, 0x05AA, "The source has changed!");

		let a: u32 = 0x05AA0000;
		let _b = a.get_u16(5, 3).unwrap();
		assert_eq!(a, 0x05AA0000, "The source has changed!");

		let a: u64 = 0x05AA00000000;
		let _b = a.get_u16(5, 3).unwrap();
		assert_eq!(a, 0x05AA00000000, "The source has changed!");

		let a: i8 = 0x05;
		let _b = a.get_i16(3, 4).unwrap();
		assert_eq!(a, 0x05, "The source has changed!");

		let a: i16 = 0x05AA;
		let _b = a.get_i16(5, 3).unwrap();
		assert_eq!(a, 0x05AA, "The source has changed!");

		let a: i32 = 0x05AA0000;
		let _b = a.get_i16(5, 3).unwrap();
		assert_eq!(a, 0x05AA0000, "The source has changed!");

		let a: i64 = 0x05AA00000000;
		let _b = a.get_i16(5, 3).unwrap();
		assert_eq!(a, 0x05AA00000000, "The source has changed!");
	}

	#[test]
	fn correct_results() {
		//
		// 8 bit input
		//

		// Same size unsigned
		let a: u8 = 0b0000_0101;
		let b = a.get_u8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// Same size signed
		let b = a.get_i8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is larger and unsigned
		let b = a.get_u16(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is larger and unsigned
		let b = a.get_u32(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is larger and unsigned
		let b = a.get_u64(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is larger and signed
		let b = a.get_i16(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is larger and signed
		let b = a.get_i32(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is larger and signed
		let b = a.get_i64(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		//
		// 16 bit input
		//

		let a: u16 = 0b0000_0101_1010_1010;
		let b = a.get_u16(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and unsigned. Pick a bit range on the left side
		let b = a.get_u8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the left side
		let b = a.get_i8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is larger and unsigned
		let b = a.get_u32(5, 3).unwrap(); // extracted bits = 101. b is u32
		assert_eq!(b, 5);

		// the type of the result is larger and unsigned
		let b = a.get_u64(5, 3).unwrap(); // extracted bits = 101. b is u64
		assert_eq!(b, 5);

		// the type of the result is larger and signed
		let b = a.get_i32(5, 3).unwrap(); // extracted bits = 101. b is u32
		assert_eq!(b, -3);

		// the type of the result is larger and signed
		let b = a.get_i64(5, 3).unwrap(); // extracted bits = 101. b is u64
		assert_eq!(b, -3);

		//
		// 32 bit input
		//

		let a: u32 = 0b0000_0101_1010_1010_1010_1010_1010_1010;
		let b = a.get_u32(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and unsigned. Pick a bit range on the left side
		let b = a.get_u8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is larger and unsigned
		let b = a.get_u64(5, 3).unwrap(); // extracted bits = 101. b is u64
		assert_eq!(b, 5);

		// the type of the result is larger and signed
		let b = a.get_i64(5, 3).unwrap(); // extracted bits = 101. b is i64
		assert_eq!(b, -3);

		//
		// 64 bit input
		//

		let a: u64 = 0b0000_0101_1010_1010_1010_1010_1010_1010_0000_0101_1010_1010_1010_1010_1010_1010;
		let b = a.get_u64(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and unsigned. Pick a bit range on the left side
		let b = a.get_u8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the left side
		let b = a.get_i8(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		//
		// Testing signed input types
		//

		let a: i32 = -1;
		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(1, 3).unwrap(); // extracted bits = 111
		assert_eq!(b, -1);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(1, 3).unwrap(); // extracted bits = 111
		assert_eq!(b, 7);

		let a: i64 = 0b0000_0101_1010_1010_1010_1010_1010_1010_0000_0101_1010_1010_1010_1010_1010_1010;
		let b = a.get_u64(5, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// TODO: Add systematic test cases for signed integers as source of data
	}

	#[test]
	fn extract_from_vector() {
		let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"

		//
		// 8 Bit
		//

		// Simple 1 for get_u8
		let bar = v.get_u8(1, 5, 3); // relevant bytes = 0x61 = 0b0110_0 --> 001 <--
		assert_eq!(bar.unwrap(), 1);

		// Simple 2 for get_u8
		let bar = v.get_u8(1, 1, 4); // relevant bytes = 0x61 = 0b0 --> 110_0 <-- 001
		assert_eq!(bar.unwrap(), 12);

		// Get a u8 from a range, which spans over 2 bytes
		let bar = v.get_u8(1, 7, 5);  // Relevant bytes = 0x61, 0x6C
		assert_eq!(bar.unwrap(), 22); // 0b0110_000 --> 1_0110 <-- _1100

		// Now signed integers

		// Simple 1 for get_i8
		let bar = v.get_i8(1, 5, 3); // relevant bytes = 0x61 = 0b0 --> 110_0 <-- 001
		assert_eq!(bar.unwrap(), 1);

		// Simple 2 for get_i8
		let bar = v.get_i8(1, 2, 3); // relevant bytes = 0x61 = 0b01 --> 10_0 <-- 001
		assert_eq!(bar.unwrap(), -4);

		// Get an i8 from a range, which spans over 2 bytes
		let bar = v.get_i8(1, 7, 5);   // Relevant bytes = 0x61, 0x6C
		assert_eq!(bar.unwrap(), -10); // 0b0110_000 --> 1_0110 <-- _1100

		//
		// 16 Bit
		//

		// Simple 1 for get_u16
		// TODO: Is this correct? : Byte offset 1 makes sure that we also test potential byte alignment issues
		let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Simple 2 for get_u16
		let bar = v.get_u16(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u16 from a range, which spans over 3 bytes
		let bar = v.get_u16(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), 728); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		// Now signed integers

		// Simple 1 for get_i16
		let bar = v.get_i16(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), -3);

		// Simple 2 for get_i16
		let bar = v.get_i16(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u16 from a range, which spans over 3 bytes
		let bar = v.get_i16(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), -296); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		//
		// 32 Bit
		//

		// Simple 1 for get_u32
		// TODO: Is this correct? : Byte offset 1 makes sure that we also test potential byte alignment issues
		let bar = v.get_u32(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Simple 2 for get_u32
		let bar = v.get_u32(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u32 from a range, which spans over 3 bytes
		let bar = v.get_u32(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), 728); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		// Now signed integers

		// Simple 1 for get_i32
		let bar = v.get_i32(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), -3);

		// Simple 2 for get_i32
		let bar = v.get_i32(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u16 from a range, which spans over 3 bytes
		let bar = v.get_i32(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), -296); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100
	}

	// TODO: add more test cases

	#[test]
	#[should_panic]
	fn panics_as_expected() {
		panic!("So far, nothing should panic!");
	}
}
