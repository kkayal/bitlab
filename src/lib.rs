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
//! 0.0.3
//!
//! # Examples
//!
//! ## Example 1: 
//! 
//! To extract 3 bits starting at bit index 5 within a byte (0xFF) and interpret them as an unsigned integer
//!
//! ```rust
//! use bitlab::*;
//! let c = 0xFFu8;
//! let b = extract_u8(c, 5, 3).unwrap();
//! assert_eq!(b, 7);
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
//! let bar = u16(&v, 1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
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
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u8))
///
/// Parameters:
///
/// - **source** (u8) an 8 bit unsigned integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_u8(source: u8, start: usize, length: usize) -> Result<u8, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<u8>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u16))
///
/// Parameters:
///
/// - **source** (u16) an 16 bit unsigned integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_u16(source: u16, start: usize, length: usize) -> Result<u16, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<u16>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u32))
///
/// Parameters:
///
/// - **source** (u32) a 32 bit unsigned integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_u32(source: u32, start: usize, length: usize) -> Result<u32, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<u32>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u64))
///
/// Parameters:
///
/// - **source** (u64) a 64 bit unsigned integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted. 
pub fn extract_u64(source: u64, start: usize, length: usize) -> Result<u64, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<u64>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i8))
///
/// Parameters:
///
/// - **source** (i8) an 8 bit signed integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_i8(source: u8, start: usize, length: usize) -> Result<i8, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<i8>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source as i8;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(u16))
///
/// Parameters:
///
/// - **source** (i16) a 16 bit signed integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_i16(source: u16, start: usize, length: usize) -> Result<i16, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<i16>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source as i16;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i32))
///
/// Parameters:
///
/// - **source** (i32) an 32 bit signed integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_i32(source: u32, start: usize, length: usize) -> Result<i32, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<i32>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source as i32;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
/// Extracts a range of bits and returns a Result object.
///
/// On success, the Result contains the desired value 
///
/// On error, the Result contains an error message.
/// This may happen if the range is larger than the data source  (start + length > sizeof(i64))
///
/// Parameters:
///
/// - **source** (i64) an 64 bit signed integer, which is the source of data  
/// - **start** (usize) the start position of the bits to be extracted. Zero is the most significant bit  
/// - **length** (usize) the number of bits to be extracted.
pub fn extract_i64(source: u64, start: usize, length: usize) -> Result<i64, (String)> {
  // Check if the desired range is valid
  let size = mem::size_of::<i64>() * 8;
  if start + length > size {
    return Err("Error while extracting bits: Out of range".to_string());
  }
 
  let mut copy = source as i64;
 
  // Lets clear the bits on both sides of the range of bits of interest
  // First clear the ones on the left side
  copy <<= start;
 
  // Second, push it all to the right end
  copy >>= size - length;
 
  // Return the result
  Ok(copy)
}
 
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

// TOOD: Implement 4 more functions for the signed integers

#[cfg(test)]
mod tests {
  use super::*;
 
  #[test]
  fn primitive_extract() {
    let a: u8 = 0b0000_0101;
    let b = extract_u8(a, 7, 1).unwrap();
    assert_eq!(b, 1);
 
    let a = 0xFF;
    let b = extract_u8(a, 5, 3).unwrap();
    assert_eq!(b, 7);
 
    let a = 0xFF_FF_FF_FFu32;
    let b = extract_u32(a, 5, 3).unwrap();
    assert_eq!(b, 7);
 
    let a = 0xFF_FF_FF_00u32;
    let b = extract_i32(a, 0, 2).unwrap();
    assert_eq!(b, -1);
    let c: i32 = -1;
    assert_eq!(b, c);
 
    let a: i16 = -3755; // = 0xF155 = 0b1111_0001_0101_0101
    let b = extract_i16(a as u16, 0, 4).unwrap();
    assert_eq!(b, -1);
    let b = extract_u16(a as u16, 0, 4).unwrap();
    assert_eq!(b, 15);

    let a = 0x7F_FF_FF_FFu32;
    let b = extract_i32(a, 0, 3).unwrap();
    assert_eq!(b, 3);
 
    let a = 0xFF_FF_FF_7Fu32;
    let b = extract_i32(a, 24, 3).unwrap();
    assert_eq!(b, 3);
 
    let a = 0xFF_FF_FF_FF_FF_FF_FF_7Fu64;
    let b = extract_i64(a, 56, 3).unwrap();
    assert_eq!(b, 3);
  }

  #[test]
  fn extract_from_vector() {
  	let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
		let bar = u16(&v, 1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 5);

		// Test integrity
		// This is still allowed
		let bar = u16(&v, 1, 7, 9); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
		assert_eq!(bar.unwrap(), 364);

		// One more bit and we are out of the game..
		match u16(&v, 1, 7, 10) {
		    Ok(_) => panic!("Nooo, this should have failed!"),
		    Err(e) => assert_eq!(e, "Out of range"),
		}

		// Check that at the end of teh vector
		match u16(&v, 4, 7, 10) {
		    Ok(_) => panic!("Nooo, this should have failed!"),
		    Err(e) => assert_eq!(e, "Out of range"),
		}

		// Check that at the end of teh vector (max valid byte_offset == 4)
		match u16(&v, 5, 2, 3) {
		    Ok(_) => panic!("Nooo, this should have failed!"),
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
