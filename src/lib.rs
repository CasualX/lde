/*!
Length Disassembler
===================

Supports `x86` and `x86_64` up to `SSE4.2`.

Valid opcodes will be length disassembled correctly. Invalid opcodes may be rejected on a best-effort basis.

## Examples

Gets the length of the first opcode in a byte slice:

```
use lde::{Isa, X64};
let result = X64::ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80");
assert_eq!(result, 2);
```

Iterates over the opcodes contained in a byte slice, returning the opcode and its virtual address:

```
let code = b"\x40\x55\x48\x83\xEC*\x00\x80";
# let mut result_opcodes = vec![&code[0..2], &code[2..6]].into_iter();
# let mut result_vas = vec![0x1000, 0x1002].into_iter();

use lde::{Isa, X64};
for (opcode, va) in X64::iter(code, 0x1000) {
	println!("{:x}: {}", va, opcode);
# 	assert_eq!(result_opcodes.next(), Some(opcode.into()));
# 	assert_eq!(result_vas.next(), Some(va));
}

// 1000: 4055
// 1002: 4883EC2A
```

Find the opcode boundary after a minimum of 5 bytes:

```
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

use lde::{Isa, X86};
let mut count = 0;
for (opcode, _) in X86::iter(INPUT_CODE, 0x1000) {
	count += opcode.len();
	if count >= 5 {
		break;
	}
}

// The answer is the first 4 opcodes, or 9 bytes

assert_eq!(count, 9);
```

Custom `Display` and `Debug` formatting including pretty printing support with the alternate flag:

```
use lde::{Isa, X64};
let iter = X64::iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);

assert_eq!(format!("{:?}", iter), "[4055] [4883EC2A] 0080");
assert_eq!(format!("{:#?}", iter), "[40 55] [48 83 EC 2A] 00 80");
assert_eq!(format!("{:}", iter), "4055\n4883EC2A\n");
assert_eq!(format!("{:#}", iter), "40 55\n48 83 EC 2A\n");
```
*/

#![no_std]
use core::{cmp, ops};

#[cfg(test)]
#[macro_use]
extern crate std;

mod contains;

mod opcode;
mod builder;
pub use self::opcode::OpCode;
pub use self::builder::OcBuilder;

mod iter;
mod iter_mut;
pub use self::iter::Iter;
pub use self::iter_mut::IterMut;

mod x86;
mod x64;

//----------------------------------------------------------------

/// Defines a type which can be safely constructed from a byte array of the same size.
///
/// Used to allow reading/writing immediates and displacements.
pub unsafe trait Int: Copy + 'static {}
unsafe impl Int for u8 {}
unsafe impl Int for u16 {}
unsafe impl Int for u32 {}
unsafe impl Int for u64 {}
unsafe impl Int for i8 {}
unsafe impl Int for i16 {}
unsafe impl Int for i32 {}
unsafe impl Int for i64 {}

/// Virtual address type.
pub trait Va: Copy + Ord + ops::Add<Output = Self> + ops::AddAssign {}
impl Va for u32 {}
impl Va for u64 {}

/// Instruction set architecture.
///
/// Defines the entry points for the length disassembler.
pub trait Isa: Sized {
	/// Virtual address type.
	type Va: Va;
	/// Returns the length of the first opcode in the given byte slice.
	///
	/// When length disassembling fails, eg. the byte slice does not contain a complete and valid instruction, the return value is `0`.
	fn ld(bytes: &[u8]) -> u32;
	/// Returns the first opcode in the byte slice if successful.
	fn peek(bytes: &[u8]) -> Option<&OpCode> {
		// The ld function guarantees that the returned length does not exceed the input byte length
		// Convince the optimizer that this indeed the case with a cmp::min
		let len = cmp::min(Self::ld(bytes) as usize, bytes.len());
		if len > 0 { Some((&bytes[..len]).into()) }
		else { None }
	}
	/// Returns the first opcode mutably in the byte slice if successful.
	fn peek_mut(bytes: &mut [u8]) -> Option<&mut OpCode> {
		// The ld function guarantees that the returned length does not exceed the input byte length
		// Convince the optimizer that this indeed the case with a cmp::min
		let len = cmp::min(Self::ld(bytes) as usize, bytes.len());
		if len > 0 { Some((&mut bytes[..len]).into()) }
		else { None }
	}
	/// Returns an iterator over the opcodes contained in the byte slice.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn iter<'a>(bytes: &'a [u8], va: Self::Va) -> Iter<'a, Self> {
		Iter { bytes, va }
	}
	/// Returns an iterator over the opcodes contained in the byte slice.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn iter_mut<'a>(bytes: &'a mut [u8], va: Self::Va) -> IterMut<'a, Self> {
		IterMut { bytes, va }
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> Self::Va;
}

//----------------------------------------------------------------

/// Length disassembler for the `x86` instruction set architecture.
pub struct X86;
impl Isa for X86 {
	type Va = u32;
	fn ld(bytes: &[u8]) -> u32 {
		x86::lde_int(bytes)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> u32 {
		len as u32
	}
}

/// Length disassembler for the `x86_64` instruction set architecture.
pub struct X64;
impl Isa for X64 {
	type Va = u64;
	fn ld(bytes: &[u8]) -> u32 {
		x64::lde_int(bytes)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> u64 {
		len as u64
	}
}
