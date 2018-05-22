/*!
Length Disassembler Engine
==========================

Supports `x86` and `x86_64` up to `SSE4.2`.

Valid opcodes will be length disassembled correctly. Invalid opcodes may be rejected on a best-effort basis.

## Examples

Get the length of the first opcode in a byte slice.

```
assert_eq!(lde::X64.ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80"), 2);
```

Iterate over the opcodes contained in a byte slice, returning the opcode and its virtual address.

```
let mut it = lde::X64.iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0x1000);
assert_eq!(it.next(), Some((b"\x40\x55".into(), 0x1000)));
assert_eq!(it.next(), Some((b"\x48\x83\xEC*".into(), 0x1002)));
assert_eq!(it.next(), None);
```

Custom `Display` and `Debug` formatting including pretty printing support with `#`.

```
let it = lde::X64.iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);
assert_eq!(format!("{:?}", it), "[4055] [4883EC2A] 0080");
assert_eq!(format!("{:#?}", it), "[40 55] [48 83 EC 2A] 00 80");
assert_eq!(format!("{:}", it), "4055\n4883EC2A\n");
assert_eq!(format!("{:#}", it), "40 55\n48 83 EC 2A\n");
```
*/

#![no_std]
use core::{cmp, ops};

#[cfg(test)]
#[macro_use]
extern crate std;

mod lde;

mod opcode;
pub use self::opcode::OpCode;

mod builder;
pub use self::builder::OpCodeBuilder;

mod iter;
pub use self::iter::Iter;

mod iter_mut;
pub use self::iter_mut::IterMut;

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
pub trait Va: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + ops::Add<Output = Self> + ops::AddAssign {}
impl Va for u32 {}
impl Va for u64 {}

/// Declares the entry point for an instruction set's length disassembler.
pub trait Isa: Sized {
	/// Virtual address type.
	type Va: Va;
	/// Length disassembles the given bytes.
	///
	/// Returns `0` on failure.
	fn ld(bytes: &[u8]) -> u32;
	/// Returns the first opcode in the byte slice if there is any.
	fn peek(bytes: &[u8]) -> Option<&OpCode> {
		// The ld function guarantees that the returned length does not exceed the input byte length
		// Convince the optimizer that this indeed the case with a a cmp::min
		let len = cmp::min(Self::ld(bytes) as usize, bytes.len());
		if len > 0 { Some((&bytes[..len]).into()) }
		else { None }
	}
	/// Returns the first opcode mutably in the byte slice if there is any.
	fn peek_mut(bytes: &mut [u8]) -> Option<&mut OpCode> {
		// The ld function guarantees that the returned length does not exceed the input byte length
		// Convince the optimizer that this indeed the case with a a cmp::min
		let len = cmp::min(Self::ld(bytes) as usize, bytes.len());
		if len > 0 { Some((&mut bytes[..len]).into()) }
		else { None }
	}
	/// Iterate over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn iter<'a>(bytes: &'a [u8], va: Self::Va) -> Iter<'a, Self> {
		Iter { bytes, va }
	}
	/// Iterate mutably over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn iter_mut<'a>(bytes: &'a mut [u8], va: Self::Va) -> IterMut<'a, Self> {
		IterMut { bytes, va }
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> Self::Va;
}

//----------------------------------------------------------------

/// Length disassembler for the `x86` instruction set.
pub struct X86;
impl Isa for X86 {
	type Va = u32;
	fn ld(bytes: &[u8]) -> u32 {
		lde::x86::lde_int(bytes)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> u32 {
		len as u32
	}
}
impl X86 {
	/// Length disassembles the given bytes.
	///
	/// Returns `0` on failure.
	pub fn ld(self, bytes: &[u8]) -> u32 {
		<X86 as Isa>::ld(bytes)
	}
	/// Returns the first opcode in the byte slice if there is any.
	pub fn peek(self, bytes: &[u8]) -> Option<&OpCode> {
		<X86 as Isa>::peek(bytes)
	}
	/// Returns the first opcode mutably in the byte slice if there is any.
	pub fn peek_mut(self, bytes: &mut [u8]) -> Option<&mut OpCode> {
		<X86 as Isa>::peek_mut(bytes)
	}
	/// Iterate over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	pub fn iter<'a>(self, bytes: &'a [u8], va: u32) -> Iter<'a, X86> {
		<X86 as Isa>::iter(bytes, va)
	}
	/// Iterate mutably over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	pub fn iter_mut<'a>(self, bytes: &'a mut [u8], va: u32) -> IterMut<'a, X86> {
		<X86 as Isa>::iter_mut(bytes, va)
	}
}

/// Length disassembler for the `x86_64` instruction set.
pub struct X64;
impl Isa for X64 {
	type Va = u64;
	fn ld(bytes: &[u8]) -> u32 {
		lde::x64::lde_int(bytes)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> u64 {
		len as u64
	}
}
impl X64 {
	/// Length disassembles the given bytes.
	///
	/// Returns `0` on failure.
	pub fn ld(self, bytes: &[u8]) -> u32 {
		<X64 as Isa>::ld(bytes)
	}
	/// Returns the first opcode in the byte slice if there is any.
	pub fn peek(self, bytes: &[u8]) -> Option<&OpCode> {
		<X64 as Isa>::peek(bytes)
	}
	/// Returns the first opcode mutably in the byte slice if there is any.
	pub fn peek_mut(self, bytes: &mut [u8]) -> Option<&mut OpCode> {
		<X64 as Isa>::peek_mut(bytes)
	}
	/// Iterate over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	pub fn iter<'a>(self, bytes: &'a [u8], va: u64) -> Iter<'a, X64> {
		<X64 as Isa>::iter(bytes, va)
	}
	/// Iterate mutably over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	pub fn iter_mut<'a>(self, bytes: &'a mut [u8], va: u64) -> IterMut<'a, X64> {
		<X64 as Isa>::iter_mut(bytes, va)
	}
}
