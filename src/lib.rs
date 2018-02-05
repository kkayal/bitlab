//![![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)
//![![Build status](https://ci.appveyor.com/api/projects/status/drb2hj2hy1bcs9ve?svg=true)](https://ci.appveyor.com/project/kkayal/bitlab)
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
//!0.5.0
//!
//!# Usage
//!
//!1. In your Cargo.toml file, add `bitlab = "0.5"` under `[dependencies]`
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
//!```rust
//!use bitlab::*;
//!let a: u8 = 0b0000_0101;
//!
//!// Get the most significant bit. It has the bit offset 0
//!assert_eq!(a.get_bit(0).unwrap(), false);
//!
//!// Set the most significant bit. Expect 0b1000_0101
//!assert_eq!(a.set_bit(0).unwrap(), 133);
//!
//!// Clear the most significant bit. Expect 0b0000_0101
//!assert_eq!(a.clear_bit(0).unwrap(), 5);
//!```
//!
//!## Example 3: 
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
//!## Example 4:
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
static LEN_TOO_BIG_MSG: &str = "The length parameter is too big for a ";

// Result-type-alias-idiom
// Source https://doc.rust-lang.org/book/first-edition/error-handling.html#the-result-type-alias-idiom
// Shortens the return type in function signatures
type Result<T> = std::result::Result<T, String>;

// Convinience macro to shorten String::from("hello") to s!("hello")
macro_rules! s {
	( $x:expr ) => { String::from($x); };
}

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
fn get_u8(self, start: usize, length: usize) -> Result<(u8)>;

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
fn get_u16(self, start: usize, length: usize) -> Result<(u16)>;

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
fn get_u32(self, start: usize, length: usize) -> Result<(u32)>;

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
fn get_u64(self, start: usize, length: usize) -> Result<(u64)>;

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
fn get_i8(self, start: usize, length: usize) -> Result<(i8)>;

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
fn get_i16(self, start: usize, length: usize) -> Result<(i16)>;

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
fn get_i32(self, start: usize, length: usize) -> Result<(i32)>;

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
fn get_i64(self, start: usize, length: usize) -> Result<(i64)>;
}

impl ExtractBitsFromIntegralTypes for u8 {
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err(s!(OUT_OF_RANGE_MSG));
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

	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

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
	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		Ok(self.get_u8(start, length)? as u16)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		Ok(self.get_i8(start, length)? as i16)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		Ok(self.get_u8(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		Ok(self.get_i8(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		Ok(self.get_u8(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		Ok(self.get_i8(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i8 {
	// TODO: Check if the type conversions below have an impact to
	// run-time performance or if it is better to optimise
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		(self as u8).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		(self as u8).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		(self as u8).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		(self as u8).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		(self as u8).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		(self as u8).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		(self as u8).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		(self as u8).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u16 {
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u16(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i16(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err(s!(OUT_OF_RANGE_MSG));
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

	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err(s!(OUT_OF_RANGE_MSG));
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
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		Ok(self.get_u16(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		Ok(self.get_i16(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		Ok(self.get_u16(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		Ok(self.get_i16(start, length)? as i64)
	}
}


impl ExtractBitsFromIntegralTypes for i16 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		(self as u16).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		(self as u16).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		(self as u16).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		(self as u16).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		(self as u16).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		(self as u16).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		(self as u16).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		(self as u16).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u32 {
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i32(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u16");
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i16");
		}

		// Return the result
		Ok(self.get_i32(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err(s!(OUT_OF_RANGE_MSG));
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

	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err(s!(OUT_OF_RANGE_MSG));
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
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		Ok(self.get_u32(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		Ok(self.get_i32(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i32 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		(self as u32).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		(self as u32).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		(self as u32).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		(self as u32).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		(self as u32).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		(self as u32).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		(self as u32).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		(self as u32).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u64 {
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u16");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i16");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		if length > 32 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u32");
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u32)
	}

	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		if length > 32 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i32");
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i32)
	}

	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err(s!(OUT_OF_RANGE_MSG));
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

	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err(s!(OUT_OF_RANGE_MSG));
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
	fn get_u8(self, start: usize, length: usize) -> Result<(u8)> {
		(self as u64).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<(i8)> {
		(self as u64).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<(u16)> {
		(self as u64).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<(i16)> {
		(self as u64).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<(u32)> {
		(self as u64).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<(i32)> {
		(self as u64).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<(u64)> {
		(self as u64).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<(i64)> {
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
	fn get_u8(&self, byte_offset: usize, start: usize, length: usize) -> Result<(u8)>;

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
	fn get_i8(&self, byte_offset: usize, start: usize, length: usize) -> Result<(i8)>;

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
	fn get_u16(&self, byte_offset: usize, start: usize, length: usize) -> Result<(u16)>;

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
	fn get_i16(&self, byte_offset: usize, start: usize, length: usize) -> Result<(i16)>;

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
	fn get_u32(&self, byte_offset: usize, start: usize, length: usize) -> Result<(u32)>;

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
	fn get_i32(&self, byte_offset: usize, start: usize, length: usize) -> Result<(i32)>;
}

impl ExtractBitsFromVecU8 for Vec<u8> {
	fn get_u8(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(u8)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}

	fn get_i8(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(i8)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}

	fn get_u16(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(u16)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}

	fn get_i16(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(i16)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}

	fn get_u32(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(u32)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}

	fn get_i32(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<(i32)> {
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
				return Err(s!(OUT_OF_RANGE_MSG))
			}
		} else {
			return Err(s!(OUT_OF_RANGE_MSG))
		}
	}
}

/// Defines a set of functions to get, set and clear single bits
pub trait SingleBits {

	/// Sets a single bit and returns a Result object, which contains the modified variable
	///
	/// On success, the Result object contains the desired value
	///
	/// On error, the Result object contains an error message.
	/// This may happen if the bit_offset is larger than the data source (bit_offset > variable size)
	///
	/// Parameters:
	///
	/// - **bit_offset** (u8) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized;

	/// Tests a single bit and returns true or false in a Result object
	///
	/// On error, the Result object contains an error message.
	/// This may happen if the bit_offset is larger than the data source (bit_offset > variable size)
	///
	/// Parameters:
	///
	/// - **bit_offset** (u8) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn get_bit(self, bit_offset: u32) -> Result<(bool)>;

	/// Clears a single bit and then returns a Result Object, which contains the modified varibale
	///
	/// On success, the Rsult object contains the desired value
	///
	/// On error, the Result object contains an error message.
	/// This may happen if the bit_offset is larger than the data source (bit_offset > variable size)
	///
	/// Parameters:
	///
	/// - **bit_offset** (u8) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized;
}

impl SingleBits for u8 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u8 = 0b0111_1111; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i8 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u8;
		copy |= a;

		Ok(copy as i8)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u8;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 7 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u8 = 0b0111_1111; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u8;
		copy &= a;

		Ok(copy as i8)
	}
}

impl SingleBits for u16 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u16 = 0b0111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i16 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u16;
		copy |= a;

		Ok(copy as i16)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u16;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 15 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u16 = 0b0111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u16;
		copy &= a;

		Ok(copy as i16)
	}
}

impl SingleBits for u32 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i32 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u32;
		copy |= a;

		Ok(copy as i32)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u32;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 31 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u32;
		copy &= a;

		Ok(copy as i32)
	}
}

impl SingleBits for u64 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u64 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i64 {
	fn set_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u64;
		copy |= a;

		Ok(copy as i64)
	}

	fn get_bit(self, bit_offset: u32) -> Result<(bool)> {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u64;
		copy = copy & a;

		if copy > 0 {
			Ok(true)
		} else {
		  Ok(false)
		}
	}

	fn clear_bit(self, bit_offset: u32) -> Result<(Self)> where Self: std::marker::Sized {
		// Check if the desired range is valid
		if bit_offset > 63 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		let a : u64 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u64;
		copy &= a;

		Ok(copy as i64)
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

		// Even if all three parameters are individually within their range,
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

	macro_rules! get_5_3 {
		( $a:ident, $x:ident, $y:expr ) => {
			let b = $a.$x(5, 3).unwrap(); // extracted bits = 101
			assert_eq!(b, $y);
		};
	}

	#[test]
	fn correct_results() {
		//
		// 8 bit input
		//

		// Same size unsigned
		let a: u8 = 0b0000_0101;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		let a: i8 = 0b0000_0101;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		//
		// 16 bit input
		//

		let a: u16 = 0b0000_0101_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		let a: i16 = 0b0000_0101_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		//
		// 32 bit input
		//

		let a: u32 = 0b0000_0101_1010_1010_1010_1010_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		let a: i32 = 0b0000_0101_1010_1010_1010_1010_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(12, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		//
		// 64 bit input
		//

		let a: u64 = 0b0000_0101_1010_1010_1010_1010_1010_1010_0000_0101_1010_1010_1010_1010_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);

		let a: i64 = 0b0000_0101_1010_1010_1010_1010_1010_1010_0000_0101_1010_1010_1010_1010_1010_1010;

		get_5_3!(a, get_u8, 5);
		get_5_3!(a, get_i8, -3);
		get_5_3!(a, get_u16, 5);
		get_5_3!(a, get_i16, -3);
		get_5_3!(a, get_u32, 5);
		get_5_3!(a, get_i32, -3);
		get_5_3!(a, get_u64, 5);
		get_5_3!(a, get_i64, -3);

		// the type of the result is smaller and signed. Pick a bit range on the right side
		let b = a.get_i8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -3);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(60, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 5);
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

	#[test]
	fn single_bits() {
		//
		// Unsigned 8 bit
		//
		let a: u8 = 0b0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(5).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 133); // Expected result = 0b1000_0101 = 133;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 69); // Expected result = 0b0100_0101 = 69;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Unsigned 16 bit
		//
		let a: u16 = 0b0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(13).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 32773); // Expected result = 0b1000_0000_0000_0101 = 32773;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 16389); // Expected result = 0b0100_0000_0000_0101 = 16389;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Unsigned 32 bit
		//
		let a: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(29).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 2_147_483_653 ); // Expected result = 0b1000_0000_0000_0000_0000_0000_0000_0101 = 2 ** 31 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 1_073_741_829); // Expected result = 0b0100_0000_0000_0000_0000_0000_0000_0101 = 2 ** 30 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Unsigned 64 bit
		//
		let a: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(61).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 0x80_00_00_00_00_00_00_05); // Expected result = 0x80_00_00_00_00_00_00_05 = 2 ** 63 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 0x40_00_00_00_00_00_00_05); // Expected result = 0x40_00_00_00_00_00_00_05 = 2 ** 62 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Signed 8 bit
		//
		let a: i8 = 0b0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(5).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), -123); // Expected result = 0b1000_0101 = 133;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 69); // Expected result = 0b0100_0101 = 69;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Signed 16 bit
		//
		let a: i16 = 0b0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(13).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), -32763); // Expected result = 0b1000_0000_0000_0101 = 32773;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 16389); // Expected result = 0b0100_0000_0000_0101 = 16389;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Signed 32 bit
		//
		let a: i32 = 0b0000_0000_0000_0000_0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(29).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), -2_147_483_643 ); // Expected result = 0b1000_0000_0000_0000_0000_0000_0000_0101 = 2 ** 31 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 1_073_741_829); // Expected result = 0b0100_0000_0000_0000_0000_0000_0000_0101 = 2 ** 30 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		//
		// Signed 64 bit
		//
		let a: i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0101;

		// Test a single bit. The most significant bit has the bit offset 0
		assert_eq!(a.get_bit(0).unwrap(), false);
		// Test an other single bit
		assert_eq!(a.get_bit(61).unwrap(), true);

		let b = 0; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), -9_223_372_036_854_775_803); // Expected result = 0x80_00_00_00_00_00_00_05 = 2 ** 63 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);

		let b = 1; // bit offset. The most significant bit has the bit offset 0

		assert_eq!(a.set_bit(b).unwrap(), 4_611_686_018_427_387_909); // Expected result = 0x40_00_00_00_00_00_00_05 = 2 ** 62 + 5;

		// Clear the same bit again
		assert_eq!(a.clear_bit(b).unwrap(), 5);
	}
}
