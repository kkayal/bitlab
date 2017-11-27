//! Extracting a range of bits  from binary data
//!
//! # Objective:
//!
//! To help developing applications, which extract bit level data from a binary source such as spacecraft telemetry
//!
//! # Status
//!
//! Experimental
//!
//! # Version
//!
//! 0.2.0
//!
//! # Examples
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
//! //assert_eq!(b, 5);
//! ```
//! 
//! ## Example 2:
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
//! # MIT Licence
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

use std::mem;

/// Defines a number of functions, which extract a range of bits from
/// primitive numeric types (u8, u16, u32 and u64, i8, i16, i32 and i64) and return
/// the result as one of the following types (u8, u16, u32 and u64, i8, i16, i32 and i64)
/// E.g. the get_u8(5,3) function extracts the bits 5,6 and 7 of
/// the variable a and returns the result as a u8 variable
pub trait ExtractBitsFromIntegralTypes {
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u8**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u16**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u32**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **u64**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i8**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i16**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i32**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)>;

/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value as a **i64**
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)>;
}

impl ExtractBitsFromIntegralTypes for u8 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err("Out of range".to_string());
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

	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err("Out of range".to_string());
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
	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		Ok(self.get_u8(start, length)? as u16)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		Ok(self.get_i8(start, length)? as i16)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		Ok(self.get_u8(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		Ok(self.get_i8(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		Ok(self.get_u8(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		Ok(self.get_i8(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i8 {
	// TODO: Check if the type conversions below have an impact to
	// run-time performance or if it is better to optimise
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		(self as u8).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		(self as u8).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		(self as u8).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		(self as u8).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		(self as u8).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		(self as u8).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		(self as u8).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		(self as u8).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u16 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u16(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i16(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err("Out of range".to_string());
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

	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err("Out of range".to_string());
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
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		Ok(self.get_u16(start, length)? as u32)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		Ok(self.get_i16(start, length)? as i32)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		Ok(self.get_u16(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		Ok(self.get_i16(start, length)? as i64)
	}
}


impl ExtractBitsFromIntegralTypes for i16 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		(self as u16).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		(self as u16).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		(self as u16).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		(self as u16).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		(self as u16).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		(self as u16).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		(self as u16).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		(self as u16).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u32 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i32(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a u16".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a i16".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err("Out of range".to_string());
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

	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err("Out of range".to_string());
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
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		Ok(self.get_u32(start, length)? as u64)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		Ok(self.get_i32(start, length)? as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i32 {
	#[inline]
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		(self as u32).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		(self as u32).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		(self as u32).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		(self as u32).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		(self as u32).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		(self as u32).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		(self as u32).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		(self as u32).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u64 {
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u8)
	}

	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i8)
	}

	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a u16".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u16)
	}

	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a i16".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i16)
	}

	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		if length > 32 {
			return Err("The length parameter is too big for a u32".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u32)
	}

	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		if length > 32 {
			return Err("The length parameter is too big for a i32".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i32)
	}

	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err("Out of range".to_string());
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

	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err("Out of range".to_string());
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
	fn get_u8(self, start: usize, length: usize) -> Result<u8, (String)> {
		(self as u64).get_u8(start, length)
	}

	#[inline]
	fn get_i8(self, start: usize, length: usize) -> Result<i8, (String)> {
		(self as u64).get_i8(start, length)
	}

	#[inline]
	fn get_u16(self, start: usize, length: usize) -> Result<u16, (String)> {
		(self as u64).get_u16(start, length)
	}

	#[inline]
	fn get_i16(self, start: usize, length: usize) -> Result<i16, (String)> {
		(self as u64).get_i16(start, length)
	}

	#[inline]
	fn get_u32(self, start: usize, length: usize) -> Result<u32, (String)> {
		(self as u64).get_u32(start, length)
	}

	#[inline]
	fn get_i32(self, start: usize, length: usize) -> Result<i32, (String)> {
		(self as u64).get_i32(start, length)
	}

	#[inline]
	fn get_u64(self, start: usize, length: usize) -> Result<u64, (String)> {
		(self as u64).get_u64(start, length)
	}

	#[inline]
	fn get_i64(self, start: usize, length: usize) -> Result<i64, (String)> {
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
	fn get_u8(&self, byte_offset: usize, start: usize, length: usize) -> Result<u8, (String)>;

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
	fn get_u16(&self, byte_offset: usize, start: usize, length: usize) -> Result<u16, (String)>;

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
	fn get_u32(&self, byte_offset: usize, start: usize, length: usize) -> Result<u32, (String)>;
}

impl ExtractBitsFromVecU8 for Vec<u8> {
	fn get_u8(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u8, String> {
		let err1 = String::from("Out of range");
		if length <= 8 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 8 {
					let mut copy: u8 = unsafe { mem::transmute_copy(&self[byte_offset]) };
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
				return Err(err1)
			}
		} else {
			return Err(err1)
		}
	}

	fn get_u16(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u16, String> {
		let err1 = String::from("Out of range");
		if length <= 16 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 16 {
					let mut copy: u16 = unsafe { mem::transmute_copy(&self[byte_offset]) };
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = u16::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset;
					// Second, push it all to the right end
					copy >>= 16 - length;
					return Ok(copy);
				} else { // The range of bits spans over 3 bytes (not more)
					// Copy the first 2 of those 3 bytes
					let mut copy1: u16 = unsafe { mem::transmute_copy(&self[byte_offset]) };

					// Take care of the byte order
					copy1 = u16::from_be(copy1);

					// Copy that into a bigger variable type
					let mut copy1_as_u32: u32 = copy1 as u32;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_u32 <<= 8;

					// Now copy the last two of the three bytes
					let mut copy2: u16 = unsafe { mem::transmute_copy(&self[byte_offset + 1]) };

					// Take care of the byte order
					copy2 = u16::from_be(copy2);

					// Logical OR these two to get the original 3 bytes
					let mut result = copy1_as_u32 | (copy2 as u32);

					// From now on, process like the normal case above
					result <<= bit_offset + 8;
					result >>= 32 - length;
					return Ok(result as u16);
				}
			} else {
				return Err(err1)
			}
		} else {
			return Err(err1)
		}
	}

	fn get_u32(&self, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u32, String> {
		let err1 = String::from("Out of range");
		if length <= 32 && bit_offset <= 7 {
			if self.len() * 8 >= byte_offset * 8 + bit_offset + length { // Ensure that we stay within the vector
				if bit_offset + length <= 32 {
					let mut copy: u32 = unsafe { mem::transmute_copy(&self[byte_offset]) };
					// Assume that the data is given in big endian and
					// convert it to whatever endiannes we have on the users machine
					copy = u32::from_be(copy);
					// Lets clear the bits on both sides of the range of bits of interest
					// First clear the ones on the left side
					copy <<= bit_offset;
					// Second, push it all to the right end
					copy >>= 32 - length;
					return Ok(copy);
				} else { // The range of bits spans over 3 bytes (not more)
					// Copy the first 2 of those 3 bytes
					let mut copy1: u32 = unsafe { mem::transmute_copy(&self[byte_offset]) };

					// Take care of the byte order
					copy1 = u32::from_be(copy1);

					// Copy that into a bigger variable type
					let mut copy1_as_u64: u64 = copy1 as u64;

					// Shift 8 bits to the left, since these are the first 2 of 3 bytes
					copy1_as_u64 <<= 8;

					// Now copy the last two of the three bytes
					let mut copy2: u32 = unsafe { mem::transmute_copy(&self[byte_offset + 1]) };

					// Take care of the byte order
					copy2 = u32::from_be(copy2);

					// Logical OR these two to get the original 3 bytes
					let mut result = copy1_as_u64 | (copy2 as u64);

					// From now on, process like the normal case above
					result <<= bit_offset + 8;
					result >>= 32 - length;
					return Ok(result as u32);
				}
			} else {
				return Err(err1)
			}
		} else {
			return Err(err1)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn range_checks() {
		//
		// Range checks for u8 as source
		//
		let a: u8 = 0x05;

		// Start is OK, Length is OK, but the sum is > 8
		match a.get_u8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		//
		// Range checks for u16 as source
		//
		let a: u16 = 0x05AA;
		match a.get_u8(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(0, 17) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		//
		// Range checks for i8 as source
		//
		let a: i8 = 0x05;

		// Start is OK, Length is OK, but the sum is > 8
		match a.get_u8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i16(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i32(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		//
		// Range checks for i16 as source
		//
		let a: i16 = 0x05AA;
		match a.get_u8(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(0, 17) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u16(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
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
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_i64(62, 4) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}
	}

	#[test]
	fn source_must_not_change() {
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

		// Simple 1 for get_u8
		let bar = v.get_u16(1, 5, 3); // relevant bytes = 0x61 = 0b0110_0 --> 001 <--
		assert_eq!(bar.unwrap(), 1);

		// Simple 2 for get_u8
		let bar = v.get_u16(1, 1, 4); // relevant bytes = 0x61 = 0b0 --> 110_0 <-- 001
		assert_eq!(bar.unwrap(), 12);

		// Get a u8 from a range, which spans over 2 bytes
		let bar = v.get_u16(1, 7, 5); // Relevant bytes = 0x61, 0x6C
		assert_eq!(bar.unwrap(), 22); // 0b0110_000 --> 1_0110 <-- _1100


		// Simple 1 for get_u16
		let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Simple 2 for get_u16
		let bar = v.get_u16(4, 3, 5); // relevant bytes = 0x6F = 0b011 --> 0_1111 <--
		assert_eq!(bar.unwrap(), 15);

		// Get a u16 from a range, which spans over 3 bytes
		let bar = v.get_u16(1, 7, 10); // Relevant bytes = 0x61, 0x6C, 0x6C
		assert_eq!(bar.unwrap(), 728); // 0b0110_000 --> 1_0110_1100_0 <-- 110_1100

		// The byte offset has to be < sizeof(vector in bytes)
		match v.get_u8(5, 2, 3) {
			Ok(_) => panic!("The range check failed to detect invalid byte offset"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// The bit offset has to be < 8
		match v.get_u8(1, 8, 10) {
			Ok(_) => panic!("The range check failed to detect invalid bit offset"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// A u8 cannot have 12 bits
		match v.get_u8(1, 5, 12) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Even if all three parametrs are individually within their range,
		// the combination might leak outside the vector
		match v.get_u8(4, 7, 5) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// A u16 cannot have 17 bits
		match v.get_u16(1, 5, 17) {
			Ok(_) => panic!("The range check failed to detect invalid length"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Even if all three parametrs are individually within their range,
		// the combination might leak outside the vector
		match v.get_u16(4, 7, 10) {
			Ok(_) => panic!("The range check failed to detect invalid range"),
			Err(e) => assert_eq!(e, "Out of range"),
		}
	}

	// TODO: add test cases for get_32 and get_64

	#[test]
	#[should_panic]
	fn panics_as_expected() {
		panic!("So far nothing should panic!");
	}
}
