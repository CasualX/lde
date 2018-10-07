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
for inst in X64::iter(code, 0x1000) {
	println!("{:x}: {:x}", inst.va(), inst);
# 	assert_eq!(result_opcodes.next(), Some(inst.bytes()));
# 	assert_eq!(result_vas.next(), Some(inst.va()));
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
// Find how many instructions need to be copied for our hook to work

use lde::{Isa, X86};
let mut count = 0;
for inst in X86::iter(INPUT_CODE, 0x1000) {
	count += inst.bytes().len();
	if count >= 5 {
		break;
	}
}

// The answer is the first 4 instructions, or 9 bytes

assert_eq!(count, 9);
```

Custom `Display` and `Debug` formatting including pretty printing support with the alternate flag:

```
use lde::{Isa, X64};
let iter = X64::iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);

assert_eq!(format!("{:?}", iter), "[4055] [4883ec2a] 0080");
assert_eq!(format!("{:#?}", iter), "[40 55] [48 83 ec 2a] 00 80");
assert_eq!(format!("{:}", iter), "4055\n4883ec2a\n");
assert_eq!(format!("{:#}", iter), "40 55\n48 83 ec 2a\n");
```
*/

#![no_std]
use core::{fmt, mem, ops, ptr, str};

#[cfg(test)]
#[macro_use]
extern crate std;

mod contains;

mod iter;
pub use self::iter::Iter;

mod x86;
mod x64;

mod inst;
pub use self::inst::*;

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

/// Helps reading immediate and displacement values.
///
/// # Examples
///
/// ```
/// // mov eax, 0x01010101
/// let opcode = b"\xB8\x01\x01\x01\x01";
///
/// // reads the immedate value
/// let result: u32 = lde::read(opcode, 1);
///
/// assert_eq!(result, 0x01010101);
/// ```
///
/// # Panics
///
/// Panics if `offset..offset + sizeof(T)` is out of bounds.
pub fn read<T: Int>(bytes: &[u8], offset: usize) -> T {
	let p = bytes[offset..offset + mem::size_of::<T>()].as_ptr() as *const T;
	unsafe { ptr::read_unaligned(p) }
}
/// Helps writing immediate and displacement values.
///
/// # Examples
///
/// ```
/// // mov al, 1
/// let mut opcode = [0xb0, 0x01];
///
/// // change the immediate to 0xff
/// lde::write(&mut opcode, 1, 0xff_u8);
///
/// assert_eq!(opcode, [0xb0, 0xff]);
/// ```
///
/// # Panics
///
/// Panics if `offset..offset + sizeof(T)` is out of bounds.
pub fn write<T: Int>(bytes: &mut [u8], offset: usize, val: T) -> &mut [u8] {
	let p = bytes[offset..offset + mem::size_of::<T>()].as_mut_ptr() as *mut T;
	unsafe { ptr::write_unaligned(p, val); }
	bytes
}

#[inline]
fn fmt_bytes(bytes: &[u8], hex_char: u8, f: &mut fmt::Formatter) -> fmt::Result {
	let mut space = false;
	for &byte in bytes.iter() {
		if space && f.alternate() {
			f.write_str(" ")?;
		}
		space = true;

		let (hi, lo) = (byte >> 4, byte & 0xf);
		let s = [
			if hi < 10 { b'0' + hi } else { hex_char + (hi - 10) },
			if lo < 10 { b'0' + lo } else { hex_char + (lo - 10) },
		];
		f.write_str(unsafe { str::from_utf8_unchecked(&s) })?;
	}
	Ok(())
}

//----------------------------------------------------------------

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
	fn ld(bytes: &[u8]) -> u32 {
		Self::inst_len(bytes).total_len as u32
	}
	/// Returns the number of prefix, opcode, argument and total bytes in the given byte slice.
	///
	/// When length disassembling fails, eg. the byte slice does not contain a complete and valid instruction, the return value is `InstLen::EMPTY`.
	fn inst_len(bytes: &[u8]) -> InstLen;
	/// Returns an iterator over the opcodes contained in the byte slice.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn iter<'a>(bytes: &'a [u8], va: Self::Va) -> Iter<'a, Self> {
		Iter { bytes, va }
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> Self::Va;
}

//----------------------------------------------------------------

/// Length disassembler for the `x86` instruction set architecture.
pub struct X86;
impl Isa for X86 {
	type Va = u32;
	fn inst_len(bytes: &[u8]) -> InstLen {
		x86::inst_len(bytes)
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
	fn inst_len(bytes: &[u8]) -> InstLen {
		x64::inst_len(bytes)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> u64 {
		len as u64
	}
}
