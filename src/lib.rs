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
//! 0.1.1
//!
//! # Examples
//!
//! ## Example 1: 
//! 
//! To extract 3 bits starting at bit index 5 within a byte (0xFF) and interpret them as an unsigned integer
//!
//! ```rust
//! use bitlab::*;
//! let a = 0xFFu8;
//! let b = a.get_u8(5, 3).unwrap();
//! //assert_eq!(b, 7);
//! ```
//! 
//! ## Example 2: 
//! 
//! The data source is a vector of u8 types. We want to go to byte offset 1, 
//! bit offset 7 and starting from there extract 3 bits as an u16
//! 
//! ```rust
//! let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
//! let bar = bitlab::u16(&v, 1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
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

/// Extracts a range of bits from a Vec<u8> and returns a Result object containing an 8 bit unsigned integer.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i64))
///
/// Parameters:
///
/// - **source** a vector of u8, which is the source of data  
/// - **byte_offset** (usize) the number of bytes to skip in source
/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
/// - **length** (usize) the number of bits to be extracted.
pub fn u8(source: &Vec<u8>, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u8, String> {
	let err1 = String::from("Out of range");
	let size = bit_offset + length;
	if size <= 8 { // Ensure that we stay within the range
		if source.len() > byte_offset {
			let mut copy: u8 = unsafe { mem::transmute_copy(&source[byte_offset]) };
		  // Lets clear the bits on both sides of the range of bits of interest
		  // First clear the ones on the left side
		  copy <<= bit_offset;
	 		// Second, push it all to the right end
	 		copy >>= 8 - length;
	 		return Ok(copy);
	 	} else {
	 		return Err(err1)
	 	}
	 } else {
	 	return Err(err1)
	 }
	}

/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 16 bit unsigned integer.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i64))
///
/// Parameters:
///
/// - **source** a vector of u8, which is the source of data  
/// - **byte_offset** (usize) the number of bytes to skip in source
/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
/// - **length** (usize) the number of bits to be extracted.
pub fn u16(source: &Vec<u8>, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u16, String> {
	let err1 = String::from("Out of range");
	let size = bit_offset + length;
	if size <= 16 {
	 	if source.len() >= byte_offset + 2 { // Ensure that we stay within the range
	 		let mut copy: u16 = unsafe { mem::transmute_copy(&source[byte_offset]) };
	 		// Assume that the data is given in big endian and
	 		// convert it to whatever endiannes we have on the users machine
	 		copy = u16::from_be(copy);
		  // Lets clear the bits on both sides of the range of bits of interest
		  // First clear the ones on the left side
		  copy <<= bit_offset;
	 		// Second, push it all to the right end
	 		copy >>= 16 - length;
	 		return Ok(copy);
	 	} else {
	 		return Err(err1)
	 	}
	 } else {
	 	return Err(err1)
	 }
	}
/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 32 bit unsigned integer.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i64))
///
/// Parameters:
///
/// - **source** a vector of u8, which is the source of data  
/// - **byte_offset** (usize) the number of bytes to skip in source
/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
/// - **length** (usize) the number of bits to be extracted.

pub fn u32(source: &Vec<u8>, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u32, String> {
	let err1 = String::from("Out of range");
	let size = bit_offset + length;
	if size <= 32 {
	 	if source.len() >= byte_offset + 4 { // Ensure that we stay within the range
	 		let mut copy: u32 = unsafe { mem::transmute_copy(&source[byte_offset]) };
	 		// Assume that the data is given in big endian and
	 		// convert it to whatever endiannes we have on the users machine
	 		copy = u32::from_be(copy);
		  // Lets clear the bits on both sides of the range of bits of interest
		  // First clear the ones on the left side
		  copy <<= bit_offset;
	 		// Second, push it all to the right end
	 		copy >>= 32 - length;
	 		return Ok(copy);
	 	} else {
	 		return Err(err1)
	 	}
	 } else {
	 	return Err(err1)
	 }
	}
/// Extracts a range of bits from a Vec<u8> and returns a Result object containing a 64 bit unsigned integer.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i64))
///
/// Parameters:
///
/// - **source** a vector of u8, which is the source of data  
/// - **byte_offset** (usize) the number of bytes to skip in source
/// - **bit_offset** (usize) the start position of the bits to be extracted. Zero is the most significant bit
/// - **length** (usize) the number of bits to be extracted.

pub fn u64(source: &Vec<u8>, byte_offset: usize, bit_offset: usize, length: usize) -> Result<u64, String> {
	let err1 = String::from("Out of range");
	let size = bit_offset + length;
	if size <= 64 {
	 	if source.len() >= byte_offset + 8 { // Ensure that we stay within the range
	 		let mut copy: u64 = unsafe { mem::transmute_copy(&source[byte_offset]) };
	 		// Assume that the data is given in big endian and
	 		// convert it to whatever endiannes we have on the users machine
	 		copy = u64::from_be(copy);
		  // Lets clear the bits on both sides of the range of bits of interest
		  // First clear the ones on the left side
		  copy <<= bit_offset;
	 		// Second, push it all to the right end
	 		copy >>= 64 - length;
	 		return Ok(copy);
	 	} else {
	 		return Err(err1)
	 	}
	 } else {
	 	return Err(err1)
	 }
	}

/// Defines a number of functions, which extract a range of bits from
/// primitive numeric types (u8, u16, u32 and u64) and return
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
		fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)>;

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
		fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)>;

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
		fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)>;

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
		fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)>;

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
		fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)>;

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
		fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)>;

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
		fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)>;

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
		fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)>;
	}

impl ExtractBitsFromIntegralTypes for u8 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		// Check if the desired range is valid
		if start + length > 8 {
			return Err("Out of range".to_string());
		}

		// Don't touch the original
		let mut copy = *self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 8 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
	  // Check if the desired range is valid
	  if start + length > 8 {
	  	return Err("Out of range".to_string());
	  }

		// Don't touch the original
		let mut copy = *self as i8;

	  // Lets clear the bits on both sides of the range of bits of interest
	  // First clear the ones on the left side
	  copy <<= start;

	  // Second, push it all to the right end
	  copy >>= 8 - length;

	  // Return the result
	  Ok(copy as i8)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		let copy = self.get_u8(start, length)?;
		Ok(copy as u16)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		let copy = self.get_i8(start, length)?;
		Ok(copy as i16)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		let copy = self.get_u8(start, length)?;
		Ok(copy as u32)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		let copy = self.get_i8(start, length)?;
		Ok(copy as i32)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		let copy = self.get_u8(start, length)?;
		Ok(copy as u64)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		let copy = self.get_i8(start, length)?;
		Ok(copy as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i8 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		(*self as u8).get_u8(start, length)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		(*self as u8).get_i8(start, length)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		(*self as u8).get_u16(start, length)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		(*self as u8).get_i16(start, length)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		(*self as u8).get_u32(start, length)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		(*self as u8).get_i32(start, length)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		(*self as u8).get_u64(start, length)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		(*self as u8).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u16 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u16(start, length)? as u8)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i16(start, length)? as i8)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		// Check if the desired range is valid
		if start + length > 16 {
			return Err("Out of range".to_string());
		}

		// Don't touch the original
		let mut copy = *self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 16 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
	  // Check if the desired range is valid
	  if start + length > 16 {
	  	return Err("Error while extracting bits: Out of range".to_string());
	  }

		// Don't touch the original
		let mut copy = *self as i16;

	  // Lets clear the bits on both sides of the range of bits of interest
	  // First clear the ones on the left side
	  copy <<= start;

	  // Second, push it all to the right end
	  copy >>= 16 - length;

	  // Return the result
	  Ok(copy as i16)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		let copy = self.get_u16(start, length)?;
		Ok(copy as u32)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		let copy = self.get_i16(start, length)?;
		Ok(copy as i32)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		let copy = self.get_u16(start, length)?;
		Ok(copy as u64)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		let copy = self.get_i16(start, length)?;
		Ok(copy as i64)
	}
}


impl ExtractBitsFromIntegralTypes for i16 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		(*self as u16).get_u8(start, length)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		(*self as u16).get_i8(start, length)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		(*self as u16).get_u16(start, length)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		(*self as u16).get_i16(start, length)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		(*self as u16).get_u32(start, length)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		(*self as u16).get_i32(start, length)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		(*self as u16).get_u64(start, length)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		(*self as u16).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u32 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u8)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i32(start, length)? as i8)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a u16".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as u16)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a i16".to_string());
		}

		// Return the result
		Ok(self.get_u32(start, length)? as i16)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		// Check if the desired range is valid
		if start + length > 32 {
			return Err("Out of range".to_string());
		}

		// Don't touch the original
		let mut copy = *self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 32 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
	  // Check if the desired range is valid
	  if start + length > 32 {
	  	return Err("Out of range".to_string());
	  }

		// Don't touch the original
		let mut copy = *self as i32;

	  // Lets clear the bits on both sides of the range of bits of interest
	  // First clear the ones on the left side
	  copy <<= start;

	  // Second, push it all to the right end
	  copy >>= 32 - length;

	  // Return the result
	  Ok(copy as i32)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		let copy = self.get_u32(start, length)?;
		Ok(copy as u64)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		let copy = self.get_i32(start, length)?;
		Ok(copy as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i32 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		(*self as u32).get_u8(start, length)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		(*self as u32).get_i8(start, length)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		(*self as u32).get_u16(start, length)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		(*self as u32).get_i16(start, length)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		(*self as u32).get_u32(start, length)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		(*self as u32).get_i32(start, length)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		(*self as u32).get_u64(start, length)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		(*self as u32).get_i64(start, length)
	}
}

impl ExtractBitsFromIntegralTypes for u64 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a u8".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u8)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		if length > 8 {
			return Err("The length parameter is too big for a i8".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i8)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a u16".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u16)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		if length > 16 {
			return Err("The length parameter is too big for a i16".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i16)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		if length > 32 {
			return Err("The length parameter is too big for a u32".to_string());
		}

		// Return the result
		Ok(self.get_u64(start, length)? as u32)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		if length > 32 {
			return Err("The length parameter is too big for a i32".to_string());
		}

		// Return the result
		Ok(self.get_i64(start, length)? as i32)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		// Check if the desired range is valid
		if start + length > 64 {
			return Err("Out of range".to_string());
		}

		// Don't touch the original
		let mut copy = *self;

		// Lets clear the bits on both sides of the range of bits of interest
		// First clear the ones on the left side
		copy <<= start;

		// Second, push it all to the right end
		copy >>= 64 - length;

		// Return the result
		Ok(copy)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
	  // Check if the desired range is valid
	  if start + length > 64 {
	  	return Err("Error while extracting bits: Out of range".to_string());
	  }

		// Don't touch the original
		let mut copy = *self as i64;

	  // Lets clear the bits on both sides of the range of bits of interest
	  // First clear the ones on the left side
	  copy <<= start;

	  // Second, push it all to the right end
	  copy >>= 64 - length;

	  // Return the result
	  Ok(copy as i64)
	}
}

impl ExtractBitsFromIntegralTypes for i64 {
	fn get_u8(&self, start: usize, length: usize) -> Result<u8, (String)> {
		(*self as u64).get_u8(start, length)
	}

	fn get_i8(&self, start: usize, length: usize) -> Result<i8, (String)> {
		(*self as u64).get_i8(start, length)
	}

	fn get_u16(&self, start: usize, length: usize) -> Result<u16, (String)> {
		(*self as u64).get_u16(start, length)
	}

	fn get_i16(&self, start: usize, length: usize) -> Result<i16, (String)> {
		(*self as u64).get_i16(start, length)
	}

	fn get_u32(&self, start: usize, length: usize) -> Result<u32, (String)> {
		(*self as u64).get_u32(start, length)
	}

	fn get_i32(&self, start: usize, length: usize) -> Result<i32, (String)> {
		(*self as u64).get_i32(start, length)
	}

	fn get_u64(&self, start: usize, length: usize) -> Result<u64, (String)> {
		(*self as u64).get_u64(start, length)
	}

	fn get_i64(&self, start: usize, length: usize) -> Result<i64, (String)> {
		(*self as u64).get_i64(start, length)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn extract_bits() {
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

		// Range check for u8
		match a.get_u8(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(5, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		match a.get_u64(0, 9) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

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

		// Range check for u16
		match a.get_u8(20, 7) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Range check for u16
		match a.get_u8(0, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

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

		// Range check for u32
		match a.get_u32(20, 30) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Range check for u32
		match a.get_u8(0, 12) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u8"),
		}

		// Range check for u32
		match a.get_u16(0, 18) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "The length parameter is too big for a u16"),
		}

		// Range check for u32
		match a.get_u64(0, 70) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

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
		let b = a.get_i8(1, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, -1);

		// the type of the result is smaller and unsigned. Pick a bit range on the right side
		let b = a.get_u8(1, 3).unwrap(); // extracted bits = 101
		assert_eq!(b, 7);

		// TODO: Add systematic test cases for signed integers as source of data
	}

	#[test]
	fn extract_from_vector() {
  	let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
		let bar = u16(&v, 1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Test integrity
		// This is still allowed
		let bar = u16(&v, 1, 7, 9);
		assert_eq!(bar.unwrap(), 364);

		// One more bit and we are out of the game..
		match u16(&v, 1, 7, 10) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Check that at the end of the vector
		match u16(&v, 4, 7, 10) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// Check that at the end of the vector (max valid byte_offset == 4)
		match u16(&v, 5, 2, 3) {
			Ok(_) => panic!("Missed the range check"),
			Err(e) => assert_eq!(e, "Out of range"),
		}

		// TODO: More test cases
	}

	#[test]
	#[should_panic]
	fn test_panic() {
		panic!("So far nothing that should panic!");
	}
}
