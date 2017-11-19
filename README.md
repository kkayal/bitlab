[![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)

# bitlab
Extracting a range of bits from binary data

# Objective:

To help developing applications, which extract bit level data from a binary source such as spacecraft telemetry

# Status

Experimental

# Documentation

This crate is published at [crates.io](https://crates.io/crates/bitlab). The detailed documentation is available at [docs.rs/bitlab](https://docs.rs/bitlab/)

# Version

0.0.4

# Installation

Just add the following line under [dependencies] to your Cargo.toml file

    bitlab = "0.0.3"

# Examples

## Example 1: 

To extract 3 bits starting at bit index 5 within a byte (0xFF) and interpret them as an unsigned integer

```rust
use bitlab::*;
let c = 0xFFu8;
let b = extract_u8(c, 5, 3).unwrap();
assert_eq!(b, 7);
```

## Example 2: 

The data source is a vector of u8 types. We want to go to byte offset 1, 
bit offset 7 and starting from there extract 3 bits as an u16

```rust
use bitlab::*;
let v: Vec<u8> = vec!{ 0x48, 0x61, 0x6C, 0x6C, 0x6F }; // = "Hallo"
let bar = u16(&v, 1, 7, 3); // relevant bytes = 0x616C = 0b0110000  --> 101 <-- 101100
assert_eq!(bar.unwrap(), 5);
```
