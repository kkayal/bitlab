//! [![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)
//! [![Build status](https://ci.appveyor.com/api/projects/status/drb2hj2hy1bcs9ve?svg=true)](https://ci.appveyor.com/project/kkayal/bitlab)
//! [![Latest Version](https://img.shields.io/crates/v/bitlab.svg)](https://crates.io/crates/bitlab)
//! 
//! # Objective:
//! 
//! To extract a range of bits from a binary data source or to insert a range of bits into a binary data structure
//! 
//! # Status
//! 
//! passively-maintained
//! 
//! # Documentation
//! 
//! This crate is published at [crates.io](https://crates.io/crates/bitlab).
//! The detailed documentation is available at [docs.rs/bitlab](https://docs.rs/bitlab/)
//! 
//! # Version
//! 
//! 1.0.0
//! 
//! ## Example 1: 
//! 
//! Start at bit offset 1, extract 3 bits and interpret the result as u8
//! 
//! ```rust
//! use bitlab::*;
//! let a: i8 = -33; // = 0b1101_1111;
//! let b = a.get_u8(1, 3).unwrap();  // 1 --> 101 <-- 1111
//! //                                         = 5
//! assert_eq!(b, 5);
//! ```
//! 
//! ## Example 2:
//! 
//! ```rust
//! use bitlab::*;
//! let a: u8 = 0b0000_0101;
//! 
//! // Get the most significant bit. It has the bit offset 0
//! assert_eq!(a.get_bit(0).unwrap(), false);
//! 
//! // Set the most significant bit. Expect 0b1000_0101
//! assert_eq!(a.set_bit(0).unwrap(), 133);
//! 
//! // Clear the most significant bit. Expect 0b0000_0101
//! assert_eq!(a.clear_bit(0).unwrap(), 5);
//! ```
//! 
//! ## Example 3: 
//! 
//! The data source is a vector of u8 types. We want to go to byte offset 1, 
//! bit offset 7 and starting from there extract 3 bits as an u16
//! 
//! ```rust
//! use bitlab::*;
//! let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
//! let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110_000  --> 1_01 <-- 10_1100
//! //                                                                         = 5
//! assert_eq!(bar.unwrap(), 5);
//! ```
//! 
//! ## Example 4:
//! 
//! Insert a 2 bit unsigned integer value (b = 3) into a variable starting at
//! the bit offset 1, where the offset = zero is the **most** significant bit.
//! 
//! ```rust
//! use bitlab::*;
//! let a : u8 = 0;
//! let b : u8 = 3;
//! let c = a.set(1, 2, b).unwrap();
//! assert_eq!(c, 0b0110_0000);
//! ```
//! 
//! ## Example 5:
//! 
//! Insert the value 3 (only 2 bits = 0b11) from a u8 into a vector
//! at byte offset = 1 and bit offset = 15
//! 
//! ```rust
//! use bitlab::*;
//! let a : u8 = 3; // = 0b0000_0011
//! let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };
//! // relevant bytes = 0x6C_6C = 0b0110_110 --> 0_0 <-- 110_1100
//! let bar = v.set(1, 15, 2, a);
//! assert_eq!(v[2], 0b0110_1101);
//! assert_eq!(v[3], 0b1110_1100);
//! ```
//! 
//! ## Example 6:
//! 
//! There is a very simple application in the examples directory,
//! which extracts the color resolution from a real gif file.
//! To run it enter the following in the command line
//! 
//! ```cli
//! cargo run --release --example gif
//! ```
//! 
//! # MIT Licence
//! 
//! Copyright <2017, Kağan Kayal>
//! 
//! Permission is hereby granted, free of charge, to any person obtaining a
//! copy of this software and associated documentation files (the "Software"),
//! to deal in the Software without restriction, including without limitation
//! the rights to use, copy, modify, merge, publish, distribute, sublicense,
//! and/or sell copies of the Software, and to permit persons to whom the
//! Software is furnished to do so, subject to the following conditions:
//! 
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//! 
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
//! EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
//! WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN\
//! CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![warn(missing_docs)]

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
html_favicon_url = "https://www.rust-lang.org/favicon.ico",
html_root_url = "https://doc.rust-lang.org/")]

//#![no_std]

// #[cfg(not(feature = "no_std"))]
// #[macro_use]
// extern crate alloc;

extern crate num;

static OUT_OF_RANGE_MSG: &str = "Out of range";
static LEN_TOO_BIG_MSG: &str = "The length parameter is too big for a ";
static LEN_ZERO: &str = "The length parameter must not be zero";

// Result-type-alias-idiom
// Source https://doc.rust-lang.org/book/first-edition/error-handling.html#the-result-type-alias-idiom
// Shortens the return type in function signatures
type Result<T> = core::result::Result<T, String>;

/// A trait to get the data type as a string for integer and floating point types.
pub trait TypeInfo {
	// Thanks to https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
	/// Returns the variable type as a string 
	fn type_of(&self) -> &'static str;
}

impl TypeInfo for u8  { fn type_of(&self) -> &'static str {"u8"}  }
impl TypeInfo for u16 { fn type_of(&self) -> &'static str {"u16"} }
impl TypeInfo for u32 { fn type_of(&self) -> &'static str {"u32"} }
impl TypeInfo for u64 { fn type_of(&self) -> &'static str {"u64"} }
impl TypeInfo for i8  { fn type_of(&self) -> &'static str {"i8"}  }
impl TypeInfo for i16 { fn type_of(&self) -> &'static str {"i16"} }
impl TypeInfo for i32 { fn type_of(&self) -> &'static str {"i32"} }
impl TypeInfo for i64 { fn type_of(&self) -> &'static str {"i64"} }
impl TypeInfo for f32 { fn type_of(&self) -> &'static str {"f32"} }
impl TypeInfo for f64 { fn type_of(&self) -> &'static str {"f64"} }

/// A trait to find out if a varibale type is signed or unsigned for integer types.
pub trait SignedInfo{
	/// Returns true if the variable is signed.
	fn is_signed(&self) -> bool;
}

impl SignedInfo for u8  { fn is_signed(&self) -> bool { false } }
impl SignedInfo for u16 { fn is_signed(&self) -> bool { false } }
impl SignedInfo for u32 { fn is_signed(&self) -> bool { false } }
impl SignedInfo for u64 { fn is_signed(&self) -> bool { false } }
impl SignedInfo for i8  { fn is_signed(&self) -> bool { true  } }
impl SignedInfo for i16 { fn is_signed(&self) -> bool { true  } }
impl SignedInfo for i32 { fn is_signed(&self) -> bool { true  } }
impl SignedInfo for i64 { fn is_signed(&self) -> bool { true  } }

// Convinience macro to shorten String::from("hello") to s!("hello")
macro_rules! s {
	( $x:expr ) => { String::from($x); };
}

macro_rules! check_max_bit_offset {
	( $x:expr ) => {
		if $x > ( core::mem::size_of::<Self>() as u32 * 8 - 1 ) as u32 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}
	}
}

macro_rules! check_range {
	( $bit_offset:expr, $length:expr ) => {
		if $length == 0 {
			return Err(s!(LEN_ZERO));
		}
		if $bit_offset + $length > core::mem::size_of::<Self>() as u32 * 8 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}
	}
}

/// How many bits does it take to write an unsigned integer?
pub fn n_required_bits_for_an_unsigned_int(num: u64) -> u32 {
	// TODO: The performance can be probaly improved by a clever lookup strategy
	let i = num as f64;
	let j = i.log2();
	if j > 0_f64 {
		j.floor() as u32 + 1
	}
	else { 1 }
}

/// How many bits does it take to write a signed integer?
pub fn n_required_bits_for_a_signed_int(num: i64) -> u32 {
	// TODO: The performance can be probaly improved by a clever lookup strategy
	let i = num as f64;
	let j = i.abs().log2();
	if j > 0_f64 {
		j.ceil() as u32 + 1
	}
	else { 1 }
}

/// Defines a number of functions, which extract a range of bits from
/// primitive numeric types (u8, u16, u32 and u64, i8, i16, i32 and i64) and return
/// the result as one of the following types (u8, u16, u32 and u64, i8, i16, i32 and i64)
/// E.g. the a.get_u8(5,3) function extracts the bits 5,6 and 7 of
/// the variable a and returns the result as a u8 variable
pub trait ExtractBitsFromIntegralTypes {
	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32>;

	/// Extracts a range of bits and returns a Result object.
	///
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64>;
}

impl ExtractBitsFromIntegralTypes for u8 {
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 8 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self as i8;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 8 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		Ok(self.get_u8 (bit_offset, length)? as u16)
	}

	#[inline]
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		Ok(self.get_i8 (bit_offset, length)? as i16)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		Ok(self.get_u8 (bit_offset, length)? as u32)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		Ok(self.get_i8 (bit_offset, length)? as i32)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		Ok(self.get_u8 (bit_offset, length)? as u64)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		Ok(self.get_i8 (bit_offset, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i8 {
	#[inline]
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		(self as u8).get_u8 (bit_offset, length)
	}

	#[inline]
	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		(self as u8).get_i8 (bit_offset, length)
	}

	#[inline]
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		(self as u8).get_u16 (bit_offset, length)
	}

	#[inline]
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		(self as u8).get_i16 (bit_offset, length)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		(self as u8).get_u32 (bit_offset, length)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		(self as u8).get_i32 (bit_offset, length)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		(self as u8).get_u64 (bit_offset, length)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		(self as u8).get_i64 (bit_offset, length)
	}
}

impl ExtractBitsFromIntegralTypes for u16 {
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u16 (bit_offset, length)? as u8)
	}

	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i16 (bit_offset, length)? as i8)
	}

	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 16 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self as i16;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 16 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		Ok(self.get_u16 (bit_offset, length)? as u32)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		Ok(self.get_i16 (bit_offset, length)? as i32)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		Ok(self.get_u16 (bit_offset, length)? as u64)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		Ok(self.get_i16 (bit_offset, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i16 {
	#[inline]
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		(self as u16).get_u8 (bit_offset, length)
	}

	#[inline]
	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		(self as u16).get_i8 (bit_offset, length)
	}

	#[inline]
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		(self as u16).get_u16 (bit_offset, length)
	}

	#[inline]
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		(self as u16).get_i16 (bit_offset, length)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		(self as u16).get_u32 (bit_offset, length)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		(self as u16).get_i32 (bit_offset, length)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		(self as u16).get_u64 (bit_offset, length)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		(self as u16).get_i64 (bit_offset, length)
	}
}

impl ExtractBitsFromIntegralTypes for u32 {
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u32 (bit_offset, length)? as u8)
	}

	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i32 (bit_offset, length)? as i8)
	}

	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u16");
		}

		// Return the result
		Ok(self.get_u32 (bit_offset, length)? as u16)
	}

	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i16");
		}

		// Return the result
		Ok(self.get_i32 (bit_offset, length)? as i16)
	}

	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 32 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self as i32;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 32 - length;

		// Return the result
		Ok(copy)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		Ok(self.get_u32 (bit_offset, length)? as u64)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		Ok(self.get_i32 (bit_offset, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i32 {
	#[inline]
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		(self as u32).get_u8 (bit_offset, length)
	}

	#[inline]
	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		(self as u32).get_i8 (bit_offset, length)
	}

	#[inline]
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		(self as u32).get_u16 (bit_offset, length)
	}

	#[inline]
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		(self as u32).get_i16 (bit_offset, length)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		(self as u32).get_u32 (bit_offset, length)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		(self as u32).get_i32 (bit_offset, length)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		(self as u32).get_u64 (bit_offset, length)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		(self as u32).get_i64 (bit_offset, length)
	}
}

impl ExtractBitsFromIntegralTypes for u64 {
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u8");
		}

		// Return the result
		Ok(self.get_u64 (bit_offset, length)? as u8)
	}

	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		if length > 8 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i8");
		}

		// Return the result
		Ok(self.get_i64 (bit_offset, length)? as i8)
	}

	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u16");
		}

		// Return the result
		Ok(self.get_u64 (bit_offset, length)? as u16)
	}

	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		if length > 16 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i16");
		}

		// Return the result
		Ok(self.get_i64 (bit_offset, length)? as i16)
	}

	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		if length > 32 {
			return Err(s!(LEN_TOO_BIG_MSG) + "u32");
		}

		// Return the result
		Ok(self.get_u64 (bit_offset, length)? as u32)
	}

	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		if length > 32 {
			return Err(s!(LEN_TOO_BIG_MSG) + "i32");
		}

		// Return the result
		Ok(self.get_i64 (bit_offset, length)? as i32)
	}

	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 64 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		// Check if the desired range is valid
		check_range!(bit_offset, length);

		// Don't touch the original
		let mut copy = self as i64;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= bit_offset;

		// Second, push it all to the right end
		copy >>= 64 - length;

		// Return the result
		Ok(copy)
	}
}

impl ExtractBitsFromIntegralTypes for i64 {
	#[inline]
	fn get_u8(self, bit_offset: u32, length: u32) -> Result<u8> {
		(self as u64).get_u8 (bit_offset, length)
	}

	#[inline]
	fn get_i8(self, bit_offset: u32, length: u32) -> Result<i8> {
		(self as u64).get_i8 (bit_offset, length)
	}

	#[inline]
	fn get_u16(self, bit_offset: u32, length: u32) -> Result<u16> {
		(self as u64).get_u16 (bit_offset, length)
	}

	#[inline]
	fn get_i16(self, bit_offset: u32, length: u32) -> Result<i16> {
		(self as u64).get_i16 (bit_offset, length)
	}

	#[inline]
	fn get_u32(self, bit_offset: u32, length: u32) -> Result<u32> {
		(self as u64).get_u32 (bit_offset, length)
	}

	#[inline]
	fn get_i32(self, bit_offset: u32, length: u32) -> Result<i32> {
		(self as u64).get_i32 (bit_offset, length)
	}

	#[inline]
	fn get_u64(self, bit_offset: u32, length: u32) -> Result<u64> {
		(self as u64).get_u64 (bit_offset, length)
	}

	#[inline]
	fn get_i64(self, bit_offset: u32, length: u32) -> Result<i64> {
		(self as u64).get_i64 (bit_offset, length)
	}
}

/// Defines a number of functions, which extract a range of bits from a Vec<u8>
/// There is one function for each variable type to be returned
/// **Important:** the contents of the vector are assumed to be **big endian** (network order)
pub trait ExtractBitsFromVecU8 {
	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 8 bit unsigned integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u8(&self, byte_offset: u32, start: u32, length: u32) -> Result<u8>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 8 bit integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i8(&self, byte_offset: u32, start: u32, length: u32) -> Result<i8>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 16 bit unsigned integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u16(&self, byte_offset: u32, start: u32, length: u32) -> Result<u16>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 16 bit integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i16(&self, byte_offset: u32, start: u32, length: u32) -> Result<i16>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 32 bit unsigned integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_u32(&self, byte_offset: u32, start: u32, length: u32) -> Result<u32>;

	/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a signed 32 bit integer or an error message.
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip in source
	/// - **bit_offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be extracted.
	fn get_i32(&self, byte_offset: u32, start: u32, length: u32) -> Result<i32>;

	// TODO: Add get_u64 and get_i64
}

impl ExtractBitsFromVecU8 for Vec<u8> {
	fn get_u8(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<u8> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 8 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					let mut copy: u8 = self[byte_offset_copy as usize];
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = u8::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset_copy;
					// Second, push it all to the right end
					copy >>= 8 - length;
					return Ok(copy);
				} else { // The range of bits spans over 2 bytes (not more)
					// Copy the first byte
					let copy1: u8 = self[byte_offset_copy as usize];

					// Copy that into a bigger variable type
					let mut copy1_as_u16: u16 = copy1 as u16;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_u16 <<= 8;

					// Now copy the second bytes
					let copy2: u8 = self[byte_offset_copy  as usize + 1];

					// Logical OR these two to get the original 2 bytes
					let mut result = copy1_as_u16 | (copy2 as u16);

					// From now on, process like the normal case above
					result <<= bit_offset_copy;
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

	fn get_i8(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<i8> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 8 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					let mut copy: i8 = self[byte_offset_copy as usize] as i8;
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = i8::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset_copy;
					// Second, push it all to the right end
					copy >>= 8 - length;
					return Ok(copy);
				} else { // The range of bits spans over 2 bytes (not more)
					// Copy the first byte
					let copy1: i8 = self[byte_offset_copy as usize] as i8;

					// Copy that into a bigger variable type
					let mut copy1_as_i16: i16 = copy1 as i16;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_i16 <<= 8;

					// Now copy the second bytes
					let copy2: i8 = self[byte_offset_copy as usize + 1] as i8;

					// Logical OR these two to get the original 2 bytes
					let mut result = copy1_as_i16 | (copy2 as i16);

					// From now on, process like the normal case above
					result <<= bit_offset_copy;
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

	fn get_u16(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<u16> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 16 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset_copy as usize] as i8;

					// Expand to u16
					let mut copy2 = copy1 as u16;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 8 + bit_offset_copy;

					// Second, push it all to the right end
					copy2 >>= 16 - length;

					return Ok(copy2);
				} else if bit_offset_copy + length <= 16 {
					let mut copy1 = self[byte_offset_copy as usize] as u16;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset_copy as usize + 1] as u16;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset_copy;

					// Second, push it all to the right end
					copy3 >>= 16 - length;

					return Ok(copy3);
				} else { // The range of bits spans over 3 bytes (not more)
					let mut copy1 = self[byte_offset_copy as usize] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset_copy as usize + 1] as u32;
					copy2 <<= 8;

					let copy3 = self[byte_offset_copy as usize + 2] as u32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset_copy + 8;

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

	fn get_i16(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<i16> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 16 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset_copy as usize] as i8;

					// Expand to i16
					let mut copy2 = copy1 as i16;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 8 + bit_offset_copy;

					// Second, push it all to the right end
					copy2 >>= 16 - length;

					return Ok(copy2);
				} else if bit_offset_copy + length <= 16 {
					let mut copy1 = self[byte_offset_copy as usize] as i16;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset_copy as usize + 1] as i16;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset_copy;

					// Second, push it all to the right end
					copy3 >>= 16 - length;

					return Ok(copy3);
				} else { // The range of bits spans over 3 bytes (not more)
					let mut copy1 = self[byte_offset_copy as usize] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset_copy as usize + 1] as i32;
					copy2 <<= 8;

					let copy3 = self[byte_offset_copy as usize + 2] as i32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset_copy + 8;

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

	fn get_u32(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<u32> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 32 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset_copy as usize] as u8;

					// Expand to u32
					let mut copy2 = copy1 as u32;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 24 + bit_offset_copy;

					// Second, push it all to the right end
					copy2 >>= 32 - length;

					return Ok(copy2);
				} else if bit_offset_copy + length <= 16 {
					let mut copy1 = self[byte_offset_copy as usize] as u32;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset_copy as usize + 1] as u32;
					// copy2 <<= 0;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset_copy + 16;

					// Second, push it all to the right end
					copy3 >>= 32 - length;

					return Ok(copy3);
				} else if bit_offset_copy + length <= 24 {
					let mut copy1 = self[byte_offset_copy as usize] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset_copy as usize + 1] as u32;
					copy2 <<= 8;

					let copy3 = self[byte_offset_copy as usize + 2] as u32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset_copy + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as u32);
				} else if bit_offset_copy + length <= 32 {
					let mut copy1 = self[byte_offset_copy as usize] as u32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 24;

					let mut copy2 = self[byte_offset_copy as usize + 1] as u32;
					copy2 <<= 16;

					let mut copy3 = self[byte_offset_copy as usize + 2] as u32;
					copy3 <<= 8;

					let copy4 = self[byte_offset_copy as usize + 3] as u32;
					// copy4 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy5 = copy1 | copy2 | copy3 | copy4;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy5 <<= bit_offset_copy;

					// Second, push it all to the right end
					copy5 >>= 32 - length;

					return Ok(copy5 as u32);
				} else {
					let mut copy1 = self[byte_offset_copy as usize] as u64;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 32;

					let mut copy2 = self[byte_offset_copy as usize + 1] as u64;
					copy2 <<= 24;

					let mut copy3 = self[byte_offset_copy as usize + 2] as u64;
					copy3 <<= 16;

					let mut copy4 = self[byte_offset_copy as usize + 3] as u64;
					copy4 <<= 8;

					let copy5 = self[byte_offset_copy as usize + 4] as u64;
					// copy5 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy6 = copy1 | copy2 | copy3 | copy4 | copy5;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy6 <<= 24 + bit_offset_copy;

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

	fn get_i32(&self, byte_offset: u32, bit_offset: u32, length: u32) -> Result<i32> {
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if length <= 32 {
			if self.len() as u32 * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				// if the bit offset is > 7 increase the byte offset as needed and reduce the bit offset until bit offset is <= 7
				let mut byte_offset_copy = byte_offset;
				let mut bit_offset_copy = bit_offset;

				byte_offset_copy += bit_offset_copy / 8;			// Integer division!
				bit_offset_copy -= (bit_offset_copy / 8) * 8;

				if bit_offset_copy + length <= 8 {
					// Don't touch the original
					let copy1 = self[byte_offset_copy as usize] as i8;

					// Expand to i32
					let mut copy2 = copy1 as i32;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy2 <<= 24 + bit_offset_copy;

					// Second, push it all to the right end
					copy2 >>= 32 - length;

					return Ok(copy2);
				} else if bit_offset_copy + length <= 16 {
					let mut copy1 = self[byte_offset_copy as usize] as i32;

					// This is the most significant byte. SO move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 8;

					let copy2 = self[byte_offset_copy as usize + 1] as i32;
					// copy2 <<= 0;

					// Logical OR these two to get the original 2 bytes
					let mut copy3 = copy1 | copy2;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy3 <<= bit_offset_copy + 16;

					// Second, push it all to the right end
					copy3 >>= 32 - length;

					return Ok(copy3);
				} else if bit_offset_copy + length <= 24 {
					let mut copy1 = self[byte_offset_copy as usize] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 16;

					let mut copy2 = self[byte_offset_copy as usize + 1] as i32;
					copy2 <<= 8;

					let copy3 = self[byte_offset_copy as usize + 2] as i32;
					// copy3 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy4 = copy1 | copy2 | copy3;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy4 <<= bit_offset_copy + 8;

					// Second, push it all to the right end
					copy4 >>= 32 - length;

					return Ok(copy4 as i32);
				} else if bit_offset_copy + length <= 32 {
					let mut copy1 = self[byte_offset_copy as usize] as i32;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 24;

					let mut copy2 = self[byte_offset_copy as usize + 1] as i32;
					copy2 <<= 16;

					let mut copy3 = self[byte_offset_copy as usize + 2] as i32;
					copy3 <<= 8;

					let copy4 = self[byte_offset_copy as usize + 3] as i32;
					// copy4 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy5 = copy1 | copy2 | copy3 | copy4;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy5 <<= bit_offset_copy;

					// Second, push it all to the right end
					copy5 >>= 32 - length;

					return Ok(copy5 as i32);
				} else {
					let mut copy1 = self[byte_offset_copy as usize] as i64;

					// This is the most significant byte. So move it to the left
					// NOTE: The byte order should be OK for both big and little endians
					copy1 <<= 32;

					let mut copy2 = self[byte_offset_copy as usize + 1] as i64;
					copy2 <<= 24;

					let mut copy3 = self[byte_offset_copy as usize + 2] as i64;
					copy3 <<= 16;

					let mut copy4 = self[byte_offset_copy as usize + 3] as i64;
					copy4 <<= 8;

					let copy5 = self[byte_offset_copy as usize + 4] as i64;
					// copy5 <<= 0;

					// Logical OR these two to get the original 3 bytes
					let mut copy6 = copy1 | copy2 | copy3 | copy4 | copy5;

					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy6 <<= 24 + bit_offset_copy;

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
	/// Parameters:
	///
	/// - **bit_offset** (u32) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized;

	/// Tests a single bit and returns true or false in a Result object
	///
	/// On error, the Result object contains an error message.
	/// This may happen if the bit_offset is larger than the data source (bit_offset > variable size)
	///
	/// Parameters:
	///
	/// - **bit_offset** (u32) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn get_bit(self, bit_offset: u32) -> Result<bool>;

	/// Clears a single bit and then returns a Result Object, which contains the modified varibale
	///
	/// Parameters:
	///
	/// - **bit_offset** (u32) the offset of the bit to be set. Zero is the **MOST** significant bit.
	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized;
}

impl SingleBits for u8 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u8 = 0b0111_1111; // Only the most significant bit is clear.

		// Rotate it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i8 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u8 = 0b1000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u8;
		copy |= a;

		Ok(copy as i8)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u8 = 0b0111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u8;
		copy &= a;

		Ok(copy as i8)
	}
}

impl SingleBits for u16 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u16 = 0b0111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i16 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u16 = 0b1000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u16;
		copy |= a;

		Ok(copy as i16)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u16 = 0b0111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u16;
		copy &= a;

		Ok(copy as i16)
	}
}

impl SingleBits for u32 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i32 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u32;
		copy |= a;

		Ok(copy as i32)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u32;
		copy &= a;

		Ok(copy as i32)
	}
}

impl SingleBits for u64 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self;
		copy |= a;

		Ok(copy)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u64 = 0b0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self;
		copy &= a;

		Ok(copy)
	}
}

impl SingleBits for i64 {
	fn set_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let mut a : u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000; // Only the most significant bit is set.

		// Shift it to the right according to the desired offset
		a >>= bit_offset;

		let mut copy = self as u64;
		copy |= a;

		Ok(copy as i64)
	}

	fn get_bit(self, bit_offset: u32) -> Result<bool> {
		check_max_bit_offset!(bit_offset);

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

	fn clear_bit(self, bit_offset: u32) -> Result<Self> where Self: core::marker::Sized {
		check_max_bit_offset!(bit_offset);

		let a : u64 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // Only the most significant bit is clear.

		// Shift it to the right according to the desired offset
		let a = a.rotate_right(bit_offset);

		let mut copy = self as u64;
		copy &= a;

		Ok(copy as i64)
	}
}

/// Provides a single function to insert a sized integer into an other sized integer type
pub trait InsertIntoSizedIntegerTypes {
	/// Inserts a sized integer value into an other sized integer type
	/// Parameters:
	///
	/// - **bit offset** (u32) the start position of the bits to be extracted. Zero is the most significant bit  
	/// - **length** (u32) the number of bits to be extracted (at the least significant side).
	/// - **value** (Any sized integer type) the value to be inserted.
	fn set<T>(self, bit_offset: u32, length: u32, value: T) -> Result<Self>
		where Self: core::marker::Sized, T: core::marker::Sized, T: SignedInfo,
		T: num::cast::AsPrimitive<u8>, T: num::cast::AsPrimitive<i8>,
		T: num::cast::AsPrimitive<u16>, T: num::cast::AsPrimitive<i16>,
		T: num::cast::AsPrimitive<u32>, T: num::cast::AsPrimitive<i32>,
		T: num::cast::AsPrimitive<u64>, T: num::cast::AsPrimitive<i64>,
		T: std::string::ToString;
}

// The first parameter ($t) is the variable type to be inserted ($t)
macro_rules! def_set_fn {
	($t:ty) => (
		fn set<T>(self, bit_offset: u32, length: u32, value: T) -> Result<Self>
		where Self: core::marker::Sized, T: core::marker::Sized, T: SignedInfo,
		T: num::cast::AsPrimitive<u8>, T: num::cast::AsPrimitive<i8>,
		T: num::cast::AsPrimitive<u16>, T: num::cast::AsPrimitive<i16>,
		T: num::cast::AsPrimitive<u32>, T: num::cast::AsPrimitive<i32>,
		T: num::cast::AsPrimitive<u64>, T: num::cast::AsPrimitive<i64>,
		T: std::string::ToString {
			// Range checks
			if length > core::mem::size_of::<Self>() as u32 * 8 {
				return Err(s!(LEN_TOO_BIG_MSG) + TypeInfo::type_of(&self));
			}

			check_range!(bit_offset, length);

			if value.is_signed() {
				let n = n_required_bits_for_a_signed_int(value.as_()); // value.as_() is type casting to u32 in this case
				if n > length {
					return Err(format!("Failed to insert {} as a {} bit signed integer variable, since it requires at least {} bits.",
						&value.to_string(), &length.to_string(), &n.to_string()))
				}
			} else {
				let n = n_required_bits_for_an_unsigned_int(value.as_()); // value.as_() is type casting to u32 in this case
				if n > length {
					return Err(format!("Failed to insert {} as a {} bit unsigned integer variable, since it requires at least {} bits.",
						&value.to_string(), &length.to_string(), &n.to_string()))
				}
			}

			let mut result = self;

			// makes sure that value_copy has the same size by type casting to Self
			let mut value_copy : Self = value.as_();
			let shift = core::mem::size_of_val(&value_copy) as u8 * 8 - (bit_offset + length) as u8;
			value_copy <<= shift;

			for i in bit_offset .. bit_offset + length {
				if value_copy.get_bit(i as u32)? {
					result = result.set_bit(i as u32)?;
				} else {
					result = result.clear_bit(i as u32)?;
				}
			}
			Ok(result)
		}
	)
}

impl InsertIntoSizedIntegerTypes for u8  { def_set_fn!(u8); }
impl InsertIntoSizedIntegerTypes for i8  { def_set_fn!(i8); }
impl InsertIntoSizedIntegerTypes for u16 { def_set_fn!(u8); }
impl InsertIntoSizedIntegerTypes for i16 { def_set_fn!(i8); }
impl InsertIntoSizedIntegerTypes for u32 { def_set_fn!(u8); }
impl InsertIntoSizedIntegerTypes for i32 { def_set_fn!(i8); }
impl InsertIntoSizedIntegerTypes for u64 { def_set_fn!(u8); }
impl InsertIntoSizedIntegerTypes for i64 { def_set_fn!(i8); }

/// Defines a functions, which inserts a range of bits into a Vec<u8>
/// **Important:** the contents of the vector are assumed to be **big endian** (network order)
pub trait InsertBitsIntoVecU8 {
	/// inserts a range of bits into a Vec<u8>
	///
	/// Parameters:
	///
	/// - **byte_offset** (u32) the number of bytes to skip
	/// - **bit_offset** (u32) the number of bits to skip. Zero is the most significant bit
	/// - **length** (u32) the number of bits to be inserted.
	/// - **value** (u32) the value to be inserted.
	fn set<T>(&mut self, byte_offset: u32, bit_offset: u32, length: u32, value: T) -> Result<()>
		where Self: core::marker::Sized, T: core::marker::Sized, T: SignedInfo,
		T: num::cast::AsPrimitive<u8>, T: num::cast::AsPrimitive<i8>,
		T: num::cast::AsPrimitive<u16>, T: num::cast::AsPrimitive<i16>,
		T: num::cast::AsPrimitive<u32>, T: num::cast::AsPrimitive<i32>,
		T: num::cast::AsPrimitive<u64>, T: num::cast::AsPrimitive<i64>,
		T: std::string::ToString, T: SingleBits + Copy;
}

impl InsertBitsIntoVecU8 for Vec<u8> {
	fn set<T>(&mut self, byte_offset: u32, bit_offset: u32, length: u32, value: T) -> Result<()>
		where Self: core::marker::Sized, T: core::marker::Sized, T: SignedInfo,
		T: num::cast::AsPrimitive<u8>, T: num::cast::AsPrimitive<i8>,
		T: num::cast::AsPrimitive<u16>, T: num::cast::AsPrimitive<i16>,
		T: num::cast::AsPrimitive<u32>, T: num::cast::AsPrimitive<i32>,
		T: num::cast::AsPrimitive<u64>, T: num::cast::AsPrimitive<i64>,
		T: std::string::ToString, T: SingleBits + Copy {

		// Range checks
		if length == 0 { return Err(s!(LEN_ZERO)); };

		if byte_offset * 8 + bit_offset + length > self.len() as u32 * 8 {
			return Err(s!(OUT_OF_RANGE_MSG));
		}

		if value.is_signed() {
			let n = n_required_bits_for_a_signed_int(value.as_()); // value.as_() is type casting to u32 in this case
			if n > length {
				return Err(format!("Failed to insert {} as a {} bit signed integer variable, since it requires at least {} bits.",
					&value.to_string(), &length.to_string(), &n.to_string()))
			}
		} else {
			let n = n_required_bits_for_an_unsigned_int(value.as_()); // value.as_() is type casting to u32 in this case
			if n > length {
				return Err(format!("Failed to insert {} as a {} bit unsigned integer variable, since it requires at least {} bits.",
					&value.to_string(), &length.to_string(), &n.to_string()))
			}
		}

		let first_relevant_byte_index = byte_offset + bit_offset / 8;
		let last_relevant_byte_index  = byte_offset + (bit_offset + length - 1) / 8;
		// For each relevant byte in the vector
		// 1. Make a copy of a byte
		// 2. For each relevant bit in the copy, set or clear the relevant bits (bit by bit)
		// 3. Replace the oríginal byte in the vector with the modified copy
		let mut bit_counter = length;
		let mut read_bit_index = core::mem::size_of::<T>() as u32 * 8 - length;
		let mut write_bit_index = bit_offset % 8;

		for byte_index in first_relevant_byte_index .. last_relevant_byte_index + 1 {
			let mut copy = self[byte_index as usize];	// Step 1

			while bit_counter > 0 {	// Step 2
				if value.get_bit(read_bit_index)? {
					copy = copy.set_bit(write_bit_index)?;
				} else {
					copy = copy.clear_bit(write_bit_index)?;
				}
				read_bit_index += 1;
				write_bit_index += 1;
				bit_counter -= 1;
				if write_bit_index % 8 == 0 {
					write_bit_index = 0;
					break;
				}
			}

			self[byte_index as usize] = copy;	// Step 3
		}

		Ok(())
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
	fn test_number_of_bits_required_for_an_unsinged_integer() {
		assert_eq!(n_required_bits_for_an_unsigned_int(0), 1);
		assert_eq!(n_required_bits_for_an_unsigned_int(1), 1);
		assert_eq!(n_required_bits_for_an_unsigned_int(2), 2);
		assert_eq!(n_required_bits_for_an_unsigned_int(3), 2);
		assert_eq!(n_required_bits_for_an_unsigned_int(4), 3);
		assert_eq!(n_required_bits_for_an_unsigned_int(5), 3);
		assert_eq!(n_required_bits_for_an_unsigned_int(6), 3);
		assert_eq!(n_required_bits_for_an_unsigned_int(7), 3);
		assert_eq!(n_required_bits_for_an_unsigned_int(8), 4);
		assert_eq!(n_required_bits_for_an_unsigned_int(255), 8);
		assert_eq!(n_required_bits_for_an_unsigned_int(256), 9);
	}

	#[test]
	fn test_number_of_bits_required_for_a_singed_integer() {
		assert_eq!(n_required_bits_for_a_signed_int(0), 1);
		assert_eq!(n_required_bits_for_a_signed_int(-1), 1);
		assert_eq!(n_required_bits_for_a_signed_int(-2), 2);
		assert_eq!(n_required_bits_for_a_signed_int(-3), 3);
		assert_eq!(n_required_bits_for_a_signed_int(-4), 3);
		assert_eq!(n_required_bits_for_a_signed_int(-5), 4);
		assert_eq!(n_required_bits_for_a_signed_int(-6), 4);
		assert_eq!(n_required_bits_for_a_signed_int(-7), 4);
		assert_eq!(n_required_bits_for_a_signed_int(-8), 4);
		assert_eq!(n_required_bits_for_a_signed_int(-63), 7);
		assert_eq!(n_required_bits_for_a_signed_int(-64), 7);
		assert_eq!(n_required_bits_for_a_signed_int(-65), 8);
		assert_eq!(n_required_bits_for_a_signed_int(-127), 8);
		assert_eq!(n_required_bits_for_a_signed_int(-128), 8);
		assert_eq!(n_required_bits_for_a_signed_int(-129), 9);
	}

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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_i8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		//
		// Range checks for u32 as source
		//

		let a: u32 = 0x05AAAAAA;

		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u16"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i16"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u16"),
		}

		match a.get_u32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u32"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i16"),
		}

		match a.get_i32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i32"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_i8(2, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		//
		// Range checks for i32 as source
		//

		let a: i32 = 0x05AAAAAA;

		match a.get_u8(20, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u16"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i16"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u8"),
		}

		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u16"),
		}

		match a.get_u32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "u32"),
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
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i8"),
		}

		match a.get_i16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i16"),
		}

		match a.get_i32(0, 33) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, s!(LEN_TOO_BIG_MSG) + "i32"),
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
		let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };

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

		// Use a large bit offset
		let bar = v.get_u8(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), 7); // 0b0110_ --> 111 <-- 1

		// Use a large bit offset, which spans over 2 bytes
		let bar = v.get_u8(0, 30, 3);   // Relevant bytes = 0x6C, 0x6F
		assert_eq!(bar.unwrap(), 0); // 0b_0110_11 --> 00_0 <-- 110_1111

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

		// Use a large bit offset
		let bar = v.get_i8(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), -1); // 0b0110_ --> 111 <-- 1

		//
		// 16 Bit
		//

		// Simple 1 for get_u16
		let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Simple 2 for get_u16
		let bar = v.get_u16(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u16 from a range, which spans over 3 bytes
		let bar = v.get_u16(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), 728); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		// Use a large bit offset
		let bar = v.get_u16(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), 7); // 0b0110_ --> 111 <-- 1

		// Now signed integers

		// Simple 1 for get_i16
		let bar = v.get_i16(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), -3);

		// Simple 2 for get_i16
		let bar = v.get_i16(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a i16 from a range, which spans over 3 bytes
		let bar = v.get_i16(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), -296); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		// Use a large bit offset
		let bar = v.get_i16(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), -1); // 0b0110_ --> 111 <-- 1

		//
		// 32 Bit
		//

		let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x65, 0x6C, 0x74, 0x21 };

		// Simple 1 for get_u32
		let bar = v.get_u32(0, 0, 3); // relevant bytes = 0x48 = 0b --> 010 <-- 0_1000
		assert_eq!(bar.unwrap(), 2);

		// Simple 2 for get_u32
		let bar = v.get_u32(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Simple 3 for get_u32
		let bar = v.get_u32(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Simple 4 for get_u32
		let bar = v.get_u32(5, 3, 16); // relevant bytes = 0x2C2057 = 0b001 --> 0_1100_0010_0000_010 <-- 1_0111
		assert_eq!(bar.unwrap(), 24834);

		// Simple 5 for get_u32
		let bar = v.get_u32(5, 3, 28); // relevant bytes = 0x2C205765 = 0b001 --> 0_1100_0010_0000_0101_0111_0110_010 <-- 1
		assert_eq!(bar.unwrap(), 101723058);

		// Simple 6 for get_u32
		let bar = v.get_u32(5, 3, 32); // relevant bytes = 0x2C2057656C = 0b001 --> 0_1100_0010_0000_0101_0111_0110_0101_011 <-- 0_1100
		assert_eq!(bar.unwrap(), 1627568939);

		// Get a u32 from a range, which spans over 5 bytes
		let bar = v.get_u32(1, 7, 26); // Relevant bytes = 0x61, 0x6C, 0x6C, 0x6F, 0x2C
		assert_eq!(bar.unwrap(), 47765726); // 0b0110_000 --> 1_0110_1100_0110_1100_0110_1111_0 <-- 010_1100

		// Use a large bit offset
		let bar = v.get_u32(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), 7); // 0b0110_ --> 111 <-- 1

		// Now signed integers

		// Simple 1 for get_i32
		let bar = v.get_i32(0, 0, 3); // relevant bytes = 0x48 = 0b --> 010 <-- 0_1000
		assert_eq!(bar.unwrap(), 2);

		// Simple 2 for get_i32
		let bar = v.get_i32(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), -3);

		// Simple 3 for get_i32
		let bar = v.get_i32(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Simple 4 for get_i32
		let bar = v.get_i32(5, 3, 16); // relevant bytes = 0x2C2057 = 0b001 --> 0_1100_0010_0000_010 <-- 1_0111
		assert_eq!(bar.unwrap(), 24834);

		// Simple 5 for get_i32
		let bar = v.get_i32(5, 3, 28); // relevant bytes = 0x2C205765 = 0b001 --> 0_1100_0010_0000_0101_0111_0110_010 <-- 1
		assert_eq!(bar.unwrap(), 101723058);

		// Simple 6 for get_i32
		let bar = v.get_i32(5, 3, 32); // relevant bytes = 0x2C2057656C = 0b001 --> 0_1100_0010_0000_0101_0111_0110_0101_011 <-- 0_1100
		assert_eq!(bar.unwrap(), 1627568939);

		// Get a i32 from a range, which spans over 5 bytes
		let bar = v.get_i32(1, 7, 26); // Relevant bytes = 0x61, 0x6C, 0x6C, 0x6F, 0x2C
		assert_eq!(bar.unwrap(), -19343138); // 0b0110_000 --> 1_0110_1100_0110_1100_0110_1111_0 <-- 010_1100

		// Use a large bit offset
		let bar = v.get_i32(0, 36, 3);   // Relevant bytes = 0x6F
		assert_eq!(bar.unwrap(), -1); // 0b0110_ --> 111 <-- 1

		//
		// 64 Bit
		//

		// TODO: Add the 64 bit tests when they are implemented
	}

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

	#[test]
	fn inserting_8_bit_vars_into_u8() {
		let a : u8 = 0;
		let b : u8 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000);

		let a : u8 = 0b0110_0011;
		let b : u8 = 0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b : u8 = 5;
		match a.set(5, 2, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert 5 as a 2 bit unsigned integer variable, since it requires at least 3 bits.")),
		}

		// b as positive signed integer
		let a : u8 = 0b0110_0011;
		let b : i8 = 0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i8 = -2;
		assert_eq!(  0b1111_1110 as u8 as i8, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b = -5;
		match a.set(5, 2, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert -5 as a 2 bit signed integer variable, since it requires at least 4 bits.")),
		}
	}

	#[test]
	fn inserting_8_bit_vars_into_u16() {
		let a : u16 = 0;
		let b : u8  = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000);

		let a : u16 = 0b0110_0011_0000_0110;
		let b : u8  = 0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0110_0011_0000_1010);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_)  => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b : u8 = 5;
		match a.set(5, 2, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert 5 as a 2 bit unsigned integer variable, since it requires at least 3 bits.")),
		}

		// b as positive signed integer
		let b : i8 =  0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i8 = -2;
		assert_eq!(  0b1111_1110 as u8 as i8, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0110_0011_0000_1010);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b = -5;
		match a.set(5, 2, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert -5 as a 2 bit signed integer variable, since it requires at least 4 bits.")),
		}
	}

	#[test]
	fn inserting_8_bit_vars_into_u32() {
		let a : u32 = 0;
		let b : u8  = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000);

		let a : u32 = 0b0110_0011_0000_0110_0110_0011_0000_0110;
		let b : u8  = 0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0110_0011_0000_0110_0110_0011_0000_1010);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_)  => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b : u8 = 5;
		match a.set(5, 2, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert 5 as a 2 bit unsigned integer variable, since it requires at least 3 bits.")),
		}

		// b as positive signed integer
		let b : i8 =  0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i8 = -2;
		assert_eq!(  0b1111_1110 as u8 as i8, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0110_0011_0000_0110_0110_0011_0000_1010);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b = -5;
		match a.set(5, 2, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert -5 as a 2 bit signed integer variable, since it requires at least 4 bits.")),
		}
	}

	#[test]
	fn inserting_8_bit_vars_into_u64() {
		let a : u64 = 0;
		let b : u8  = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		let a : u64 = 0b0110_0011_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_0000;
		let b : u8  = 0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0110_0011_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_)  => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b : u8 = 5;
		match a.set(5, 2, b) {
			Ok(_)  => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert 5 as a 2 bit unsigned integer variable, since it requires at least 3 bits.")),
		}

		// b as positive signed integer
		let b : i8 =  0b0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i8 = -2;
		assert_eq!(  0b1111_1110 as u8 as i8, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0110_0011_0000_0110_0110_0011_0000_0110_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// The _length_ parameter must not be smaller than the number of bits,
		// which is required to represent _value_
		let b = -5;
		match a.set(5, 2, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!("Failed to insert -5 as a 2 bit signed integer variable, since it requires at least 4 bits.")),
		}
	}

	#[test]
	fn inserting_16_bit_vars_into_u8() {
		let a : u8 = 0;
		let b : u16 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000);

		let a : u8 = 0b0110_0011;
		let b : u16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let a : u8 = 0b0110_0011;
		let b : i16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i16 = -2;
		assert_eq!(  0b1111_1111_1111_1110 as u16 as i16, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_16_bit_vars_into_u16() {
		let a : u16 = 0;
		let b : u16 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000);

		let a : u16 = 0b0110_0011_0000_1110;
		let b : u16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0110_0011_0000_1010);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i16 = -2;
		assert_eq!(  0b1111_1111_1111_1110 as u16 as i16, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0110_0011_0000_1010);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_16_bit_vars_into_u32() {
		let a : u32 = 0;
		let b : u16 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000);

		let a : u32 = 0b0110_0011_0000_1110_0000_0000_0000_0000;
		let b : u16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0110_0011_0000_1110_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i16 = -2;
		assert_eq!(  0b1111_1111_1111_1110 as u16 as i16, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0110_0011_0000_1110_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_16_bit_vars_into_u64() {
		let a : u64 = 0;
		let b : u16 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		let a : u64 = 0b0110_0011_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
		let b : u16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0110_0011_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i16 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i16 = -2;
		assert_eq!(  0b1111_1111_1111_1110 as u16 as i16, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0110_0011_0000_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_32_bit_vars_into_u8() {
		let a : u8 = 0;
		let b : u32 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000);

		let a : u8 = 0b0110_0011;
		let b : u32 = 0b0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let a : u8 = 0b0110_0011;
		let b : i32 = 0b0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i32 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1110 as u32 as i32, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_32_bit_vars_into_u16() {
		let a : u16 = 0;
		let b : u32 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000);

		let a : u16 = 0b0000_0000_0110_0011;
		let b : u32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0000_0000_0110_1011);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i32 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1110 as u32 as i32, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0000_0000_0110_1011);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_32_bit_vars_into_u32() {
		let a : u32 = 0;
		let b : u32 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000);

		let a : u32 = 0b0000_0000_0110_0011_0000_0000_0000_0000;
		let b : u32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i32 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1110 as u32 as i32, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_32_bit_vars_into_u64() {
		let a : u64 = 0;
		let b : u32 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		let a : u64 = 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
		let b : u32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i32 = 2;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i32 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1110 as u32 as i32, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(60, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_64_bit_vars_into_u8() {
		let a : u8 = 0;
		let b : u64 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000);

		let a : u8 = 0b0110_0011;
		let b : u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i64 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 as u64 as i64, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0110_0101);

		// You cannot insert 9 bits into an u8
		match a.set(5, 9, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u8")),
		}

		// start + length must not exceed 8 bit (size of u8)
		match a.set(5, 8, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_64_bit_vars_into_u16() {
		let a : u16 = 0;
		let b : u64 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000);

		let a : u16 = 0b0000_0000_0110_0011;
		let b : u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0000_0000_0110_1011);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i64 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 as u64 as i64, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011);

		// Use a big bit_offset
		assert_eq!(a.set(12, 2, b).unwrap(), 0b0000_0000_0110_1011);

		// You cannot insert 18 bits into an u16
		match a.set(5, 18, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u16")),
		}

		// start + length must not exceed 16 bit (size of u16)
		match a.set(5, 15, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_64_bit_vars_into_u32() {
		let a : u32 = 0;
		let b : u64 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000);

		let a : u32 = 0b0000_0000_0110_0011_0000_0000_0000_0000;
		let b : u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a comiler warning claiming out of range for an i8.
		// IMHO, the warning is wrong, since that bit pattern is a valid i8 and the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i64 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 as u64 as i64, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000);

		// Use a big bit_offset
		assert_eq!(a.set(28, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_1000);

		// You cannot insert 40 bits into an u32
		match a.set(5, 40, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u32")),
		}

		// start + length must not exceed 32 bit (size of u32)
		match a.set(5, 30, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_64_bit_vars_into_u64() {
		let a : u64 = 0;
		let b : u64 = 3;
		assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);

		let a : u64 = 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_0000;
		let b : u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_0000);

		// Use a big bit_offset
		assert_eq!(a.set(45, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0110_0000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// b as positive signed integer
		let b : i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_0000);

		// b as negative signed integer
		// Using 0b11111111 as i8 gives a warning claiming out of range for a i8.
		// IMHO, the warning is wrong, since the actual result is what I expect.
		// Using 'as u64 as i64' below is a workaround to prevent that warning.
		// This is successfully supressing the warning, but the logic behind it seems to be inconsistent to me.
		// See the (currently open) discussion at https://github.com/rust-lang/rust/issues/48073
		let b : i64 = -2;
		assert_eq!(  0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 as u64 as i64, b);
		assert_eq!(a.set(5, 2, b).unwrap(), 0b0000_0100_0110_0011_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_0000);

		// Use a big bit_offset
		assert_eq!(a.set(45, 2, b).unwrap(), 0b0000_0000_0110_0011_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0110_0000);

		// You cannot insert 80 bits into an u64
		match a.set(5, 80, b) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!(s!(LEN_TOO_BIG_MSG) + "u64")),
		}

		// start + length must not exceed 64 bit (size of u64)
		match a.set(5, 60, b) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}
	}

	#[test]
	fn inserting_into_a_vector() {
		// Simple 1: Insert 2 bits of the variable a into the vector v at byte offset 0 and bit offset 0.
		let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };
		let a : u8 = 3; // = 0b0000_0011
		let bar = v.set(0, 0, 2, a);	// relevant bytes = 0x48 = 0b --> 01 <-- 00_1000
		assert_eq!(bar.unwrap(), ());	// There were no errors
		assert_eq!(v[0], 0b1100_1000);

		// Simple 2: Insert 2 bits of the variable a into the vector v at byte offset 1 and bit offset 0.
		let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };
		let a : u8 = 3; // = 0b0000_0011
		let bar = v.set(1, 0, 2, a);	// relevant bytes = 0x61 = 0b --> 01 <-- 10_0001
		assert_eq!(bar.unwrap(), ());	// There were no errors
		assert_eq!(v[1], 0b1110_0001);

		// Complex 1: Insert 2 bits of the variable a into the vector v at byte offset 1 and bit offset 15.
		let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };
		let a : u8 = 3; // = 0b0000_0011
		let bar = v.set(1, 15, 2, a); // relevant bytes = 0x6C_6C = 0b0110_110 --> 0_0 <-- 110_1100
		assert_eq!(bar.unwrap(), ());	// There were no errors
		assert_eq!(v[2], 0b0110_1101);
		assert_eq!(v[3], 0b1110_1100);

		// Complex 2: Insert 20 bits of the variable a into the vector v at byte offset 2 and bit offset 15.
		let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x00, 0x6C, 0x6F, 0x00, 0xFF, 0x0F };
		let a : i32 = 0b0000_0000_0000_0101_0101_0101_0101_0101;
		// relevant bytes = 0x6C_6F_00_FF = 0b0110_110 --> 0_0110_1111_0000_0000_111 <-- 1_1111
		// insert the last 20 bits of a          -->       0 1010 1010 1010 1010 101
		let bar = v.set(2, 15, 20, a);
		assert_eq!(bar.unwrap(), ());	// There were no errors
		assert_eq!(v[2], 0);
		assert_eq!(v[3], 0b0110_1100);
		assert_eq!(v[4], 0b1010_1010);
		assert_eq!(v[5], 0b1010_1010);
		assert_eq!(v[6], 0b1011_1111);

		// Range check 1: Set the last bit in the vector (is allowed --> no error)
		let mut v: Vec<u8> = vec!{ 0x00, 0x00, 0x00 };
		let i = v.len() as u32 - 1; // highest index = byte offset
		let bar = v.set(i, 7, 1, 1);
		assert_eq!(bar.unwrap(), ());	// There were no errors
		assert_eq!(v[i as usize], 0x01);

		// Range check 2: Try to set the next bit
		match v.set(i, 8, 1, 1) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// Range check 3: Start within the last byte, but spill over into the next byte
		match v.set(i, 7, 2, 1) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// Range check 3: Same as the one before but using zero byte offset and a high bit offset
		match v.set(0, i * 8 + 7, 2, 1) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// Range check 4: Use a high byte offset
		match v.set(i + 1, 0, 1, 1) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, s!(OUT_OF_RANGE_MSG)),
		}

		// Range check 5: Complain if the value cannot be represented by length bits
		match v.set(0, 0, 1, 3 as u32) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, s!("Failed to insert 3 as a 1 bit unsigned integer variable, since it requires at least 2 bits.")),
		}
	}
}
