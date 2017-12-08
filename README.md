[![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)

# Objective:

To extract a range of bits from a binary data source

# Status

Experimental

# Documentation

This crate is published at [crates.io](https://crates.io/crates/bitlab). The detailed documentation is available at [docs.rs/bitlab](https://docs.rs/bitlab/)

# Version

0.4.3

# Usage

1. In your Cargo.toml file, add `bitlab = "0.4"` under `[dependencies]`
2. In your source file, add `extern crate bitlab` and `use bitlab::*;`

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

The data source is a vector of u8 types. We want to go to byte offset 1, 
bit offset 7 and starting from there extract 3 bits as an u16

```rust
use bitlab::*;
let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
let bar = v.get_u16(1, 7, 3); // relevant bytes = 0x616C = 0b0110_000  --> 1_01 <-- 10_1100
//                                                                         = 5
assert_eq!(bar.unwrap(), 5);
```

## Example 3:

There is a very simple application in the examples directory, which extracts the color resolution from a real gif file. To run it enter the folloeing in the command line

```
cargo run --release --example gif
```
