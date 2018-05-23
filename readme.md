Length Disassembler
===================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/lde.svg)](https://crates.io/crates/lde)
[![docs.rs](https://docs.rs/lde/badge.svg)](https://docs.rs/lde)
[![Build status](https://ci.appveyor.com/api/projects/status/xtw664teq0woc6j0/branch/master?svg=true)](https://ci.appveyor.com/project/CasualX/lde/branch/master)
[![Build Status](https://travis-ci.org/CasualX/lde.svg?branch=master)](https://travis-ci.org/CasualX/lde)

Given a byte slice, extract the lengths of the opcodes in it.

Supports `x86` and `x86_64`.

Library
-------

This library can be found on [crates.io](https://crates.io/crates/lde) and its documentation on [docs.rs](https://docs.rs/crate/lde/).

In your Cargo.toml put

```
[dependencies]
lde = "0.3"
```

Examples
--------

Gets the length of the first opcode in a byte slice:

```rust
let result = lde::X64.ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80");
assert_eq!(result, 2);
```

Iterates over the opcodes contained in a byte slice, returning the opcode and its virtual address:

```rust
let code = b"\x40\x55\x48\x83\xEC*\x00\x80";

for (opcode, va) in lde::X64.iter(code, 0x1000) {
	println!("{:x}: {}", va, opcode);
}

// 1000: 4055
// 1002: 4883EC2A
```

Find the opcode boundary after a minimum of 5 bytes:

```rust
// 1000: 56         push esi
// 1001: 33f6       xor esi,esi
// 1003: 57         push edi
// 1004: bfa0104000 mov edi,0x4010a0
// 1009: 85d2       test edx,edx
// 100b: 7410       je loc_0000001d
// 100d: 8bf2       mov esi,edx
// 100f: 8bfa       mov edi,edx

const INPUT_CODE: &[u8] = b"\x56\x33\xF6\x57\xBF\xA0\x10\x40\x00\x85\xD2\x74\x10\x8B\xF2\x8B\xFA";

// We'd like to overwrite the first 5 bytes with a jmp hook
// Find how many opcodes need to be copied for our hook to work

let mut count = 0;
for (opcode, _) in lde::X86.iter(INPUT_CODE, 0x1000) {
	count += opcode.len();
	if count >= 5 {
		break;
	}
}

// The answer is the first 4 opcodes, or 9 bytes

assert_eq!(count, 9);
```

Custom `Display` and `Debug` formatting including pretty printing support with the alternate flag:

```rust
let iter = lde::X64.iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);

assert_eq!(format!("{:?}", iter), "[4055] [4883EC2A] 0080");
assert_eq!(format!("{:#?}", iter), "[40 55] [48 83 EC 2A] 00 80");
assert_eq!(format!("{:}", iter), "4055\n4883EC2A\n");
assert_eq!(format!("{:#}", iter), "40 55\n48 83 EC 2A\n");
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
