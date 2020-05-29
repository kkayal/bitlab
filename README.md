[![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)
[![Build status](https://ci.appveyor.com/api/projects/status/drb2hj2hy1bcs9ve?svg=true)](https://ci.appveyor.com/project/kkayal/bitlab)
[![Latest Version](https://img.shields.io/crates/v/bitlab.svg)](https://crates.io/crates/bitlab)

# Objective:

To extract a range of bits from a binary data source or to insert a range of bits into a binary data structure

# Status

passively-maintained

# Documentation

This crate is published at [crates.io](https://crates.io/crates/bitlab). The detailed documentation is available at [docs.rs/bitlab](https://docs.rs/bitlab/)

# Version

1.0.0

## Example 1: 

Start at bit offset 1, extract 3 bits and interpret the result as u8

```rust
use bitlab::*;
let a: i8 = -33; // = 0b1101_1111;
let b = a.get_u8(1, 3).unwrap();  // 1 --> 101 <-- 1111
//                                         = 5
assert_eq!(b, 5);
```

## Example 2:

```rust
use bitlab::*;
let a: u8 = 0b0000_0101;

// Get the most significant bit. It has the bit offset 0
assert_eq!(a.get_bit(0).unwrap(), false);

// Set the most significant bit. Expect 0b1000_0101
assert_eq!(a.set_bit(0).unwrap(), 133);

// Clear the most significant bit. Expect 0b0000_0101
assert_eq!(a.clear_bit(0).unwrap(), 5);
```

## Example 3: 

The data source is a vector of u8 types. We want to go to byte offset 1, 
bit offset 7 and starting from there extract 3 bits as an u16

```rust
use bitlab::*;
let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110_000  --> 1_01 <-- 10_1100
//                                                                         = 5
assert_eq!(bar.unwrap(), 5);
```

## Example 4:

Insert a 2 bit unsigned integer value (b = 3) into a variable starting at the bit offset 1, where the offset = zero is the **most** significant bit.

```rust
use bitlab::*;
let a : u8 = 0;
let b : u8 = 3;
assert_eq!(a.set(1, 2, b).unwrap(), 0b0110_0000);
```

## Example 5:

Insert the value 3 (only 2 bits = 0b11) from a u8 into a vector
at byte offset = 1 and bit offset = 15

```rust
use bitlab::*;
let a : u8 = 3; // = 0b0000_0011
let mut v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F };
// relevant bytes = 0x6C_6C = 0b0110_110 --> 0_0 <-- 110_1100
let bar = v.set(1, 15, 2, a);
assert_eq!(v[2], 0b0110_1101);
assert_eq!(v[3], 0b1110_1100);
```

## Example 6:

There is a very simple application in the examples directory, which extracts the color resolution from a real gif file. To run it enter the folloeing in the command line

```
cargo run --release --example gif
```
