[![Travis Build Status](https://api.travis-ci.org/kkayal/bitlab.svg?branch=master)](https://travis-ci.org/kkayal/bitlab)

# bitlab
Extracting a range of bits  from binary data

# Objective:

To help developing applications, which extract bit level data from a binary source such as spacecraft telemetry

# Status

Experimental

# Version

0.0.1

# Examples

To extract 3 bits starting at bit index 5 within a byte (0xFF) and interpret them as an unsigned integer

```rust
use bitlab::*;
let c = 0xFFu8;
let b = extract_u8(c, 5, 3).unwrap();
assert_eq!(b, 7);
```
