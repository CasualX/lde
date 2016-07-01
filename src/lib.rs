/*!
# Length Disassembler Engine

Supports `x86` and `x86_64` up to `SSE4.2`.

Valid opcodes will be length disassembled correctly. Invalid opcodes may be rejected on a best-effort basis.

## Examples

Get the length of the first opcode in a byte slice.

```
use lde::{InsnSet, x64};

assert_eq!(x64::ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80"), 2);
```

Iterate over the opcodes contained in a byte slice, returning the opcode and virtual address of the opcode.

```
use lde::{InsnSet, OpCode, x64};

assert_eq!(
	x64::iter(b"\x40\x55\x48\x83\xEC\xFC\x00\x80", 0x1000)
		.collect::<Vec<_>>(),
	vec![(OpCode(b"\x40\x55"), 0x1000),
	     (OpCode(b"\x48\x83\xEC\xFC"), 0x1002)]);
```

Custom `Display` and `Debug` formatting including pretty printing support with `#`.

```
use lde::{InsnSet, x64};

let it = x64::iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);
assert_eq!(format!("{:?}", it), "[4055] [4883EC2A] 0080");
assert_eq!(format!("{:#?}", it), "[40 55] [48 83 EC 2A] 00 80");
assert_eq!(format!("{:}", it), "4055\n4883EC2A\n");
assert_eq!(format!("{:#}", it), "40 55\n48 83 EC 2A\n");
```
*/

#![no_std]
mod lde;
pub mod ext;
use ::core::{ptr, mem, fmt, ops};

//----------------------------------------------------------------

/// Declares the entry point for an instruction set's length disassembler.
pub trait InsnSet: Clone {
	/// Virtual address type.
	type Va: Copy + Clone + ops::AddAssign + ops::Add<Output = Self::Va>;
	/// Length disassemble the given bytes.
	///
	/// Returns `0` on failure.
	fn ld(codes: &[u8]) -> u32;
	/// Create an iterator over the opcodes contained in the byte slice.
	///
	/// Provide a Virtual Address to keep track of the instruction pointer.
	fn iter(codes: &[u8], va: Self::Va) -> LDIter<Self>;
	#[doc(hidden)]
	fn as_va(len: usize) -> Self::Va;
}

/// Length disassembler for the `x86` instruction set.
///
/// You'll want to import the `InsnSet` trait too.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct x86;
impl InsnSet for x86 {
	type Va = u32;
	#[inline]
	fn ld(codes: &[u8]) -> u32 {
		lde::x86::lde_int(codes)
	}
	#[inline]
	fn iter(codes: &[u8], va: Self::Va) -> LDIter<x86> {
		LDIter(codes, va)
	}
	#[inline]
	fn as_va(len: usize) -> Self::Va {
		len as Self::Va
	}
}

/// Length disassembler for the `x86_64` instruction set.
///
/// You'll want to import the `InsnSet` trait too.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct x64;
impl InsnSet for x64 {
	type Va = u64;
	#[inline]
	fn ld(codes: &[u8]) -> u32 {
		lde::x64::lde_int(codes)
	}
	#[inline]
	fn iter(codes: &[u8], va: Self::Va) -> LDIter<x64> {
		LDIter(codes, va)
	}
	#[inline]
	fn as_va(len: usize) -> Self::Va {
		len as Self::Va
	}
}

//----------------------------------------------------------------

/// Newtype wrapper for opcode byte slices.
///
/// Adds custom `Debug` and `Display` formatters to output its contents as hex bytes,
///  add the alternate flag `#` to put spaces between the hexadecimal octets.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OpCode<'a>(pub &'a [u8]);
impl<'a> OpCode<'a> {
	/// Helper for reading immediates and displacements.
	#[inline]
	pub fn read<T: Copy>(self, offset: usize) -> T {
		let target = (&self.0[offset..offset + mem::size_of::<T>()]).as_ptr() as *const T;
		unsafe { ptr::read(target) }
	}
}
impl<'a> ops::Deref for OpCode<'a> {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &[u8] {
		self.0
	}
}
impl<'a> fmt::Debug for OpCode<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		debug_hex(f, self.0)
	}
}
impl<'a> fmt::Display for OpCode<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		display_hex(f, self.0)
	}
}

/// Iterator over opcodes in a byte slice.
#[derive(Clone)]
pub struct LDIter<'a, E: InsnSet>(&'a [u8], E::Va);
impl<'a, E: InsnSet> LDIter<'a, E> {
	/// Length disassemble the current location without advancing the iterator.
	#[inline]
	pub fn peek(&self) -> Option<OpCode<'a>> {
		let len = E::ld(self.0);
		if len > 0 { Some(OpCode(&self.0[..len as usize])) }
		else { None }
	}
	/// Skip bytes from the input without length disassembling them.
	#[inline]
	pub fn consume(&mut self, n: usize) {
		let n = ::core::cmp::min(n, self.0.len());
		self.0 = &self.0[n..];
		self.1 += E::as_va(n);
	}
	/// Get the current virtual address.
	#[inline]
	pub fn va(&self) -> E::Va {
		self.1
	}
	/// Set the current virtual address.
	#[inline]
	pub fn set_va(&mut self, va: E::Va) -> E::Va {
		let old = self.1;
		self.1 = va;
		old
	}
}
impl<'a, E: InsnSet> Iterator for LDIter<'a, E> {
	type Item = (OpCode<'a>, E::Va);
	#[inline]
	fn next(&mut self) -> Option<(OpCode<'a>, E::Va)> {
		self.peek().and_then(|opcode| {
			let va = self.1;
			self.consume(opcode.len());
			Some((opcode, va))
		})
	}
}
impl<'a, E: InsnSet> ops::Deref for LDIter<'a, E> {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &[u8] {
		self.0
	}
}
impl<'a, E: InsnSet> fmt::Debug for LDIter<'a, E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut it = self.clone();
		while let Some((opcode, _)) = it.next() {
			try!(debug_hex(f, opcode.0));
			try!(write!(f, " "));
		}
		display_hex(f, it.0)
	}
}
impl<'a, E: InsnSet> fmt::Display for LDIter<'a, E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (opcode, _) in self.clone() {
			try!(display_hex(f, opcode.0));
			try!(write!(f, "\n"));
		}
		Ok(())
	}
}

fn debug_hex(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
	if let Some((byte, tail)) = bytes.split_first() {
		try!(write!(f, "[{:02X}", byte));
		for byte in tail {
			if f.flags() & 4 != 0 {
				try!(write!(f, " "));
			}
			try!(write!(f, "{:02X}", byte));
		}
	}
	else {
		try!(write!(f, "["))
	}
	write!(f, "]")
}
fn display_hex(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
	if let Some((byte, tail)) = bytes.split_first() {
		try!(write!(f, "{:02X}", byte));
		for byte in tail {
			if f.flags() & 4 != 0 {
				try!(write!(f, " "));
			}
			try!(write!(f, "{:02X}", byte));
		}
	}
	Ok(())
}

//----------------------------------------------------------------

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
	use super::*;
	use ::std::prelude::v1::*;
	#[test]
	fn display() {
		let it = x64::iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);
		assert_eq!(format!("{:?}", it), "[4055] [4883EC2A] 0080");
		assert_eq!(format!("{:#?}", it), "[40 55] [48 83 EC 2A] 00 80");
		assert_eq!(format!("{:}", it), "4055\n4883EC2A\n");
		assert_eq!(format!("{:#}", it), "40 55\n48 83 EC 2A\n");
	}
	#[test]
	fn units() {
		let it = x64::iter(b"\x40\x55\x48\x83\xEC*\x00\x80", 0x1000);
		assert_eq!(it.collect::<Vec<_>>(), vec![(OpCode(b"\x40\x55"), 0x1000), (OpCode(b"\x48\x83\xEC*"), 0x1002)]);
	}
}
