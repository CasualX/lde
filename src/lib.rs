/*!
Length Disassembler Engine
==========================

Supports `x86` and `x86_64` up to `SSE4.2`.

Valid opcodes will be length disassembled correctly. Invalid opcodes may be rejected on a best-effort basis.

## Examples

Get the length of the first opcode in a byte slice.

```
use lde::{InsnSet, x64};

assert_eq!(x64::ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80"), 2);
```

Iterate over the opcodes contained in a byte slice, returning the opcode and its virtual address.

```
use lde::{InsnSet, x64};

let mut it = x64::lde(b"\x40\x55\x48\x83\xEC*\x00\x80", 0x1000);
assert_eq!(it.next(), Some(x64::code(b"\x40\x55", 0x1000)));
assert_eq!(it.next(), Some(x64::code(b"\x48\x83\xEC*", 0x1002)));
assert_eq!(it.next(), None);
```

Custom `Display` and `Debug` formatting including pretty printing support with `#`.

```
use lde::{InsnSet, x64};

let it = x64::lde(b"\x40\x55\x48\x83\xEC*\x00\x80", 0);
assert_eq!(format!("{:?}", it), "[4055] [4883EC2A] 0080");
assert_eq!(format!("{:#?}", it), "[40 55] [48 83 EC 2A] 00 80");
assert_eq!(format!("{:}", it), "4055\n4883EC2A\n");
assert_eq!(format!("{:#}", it), "40 55\n48 83 EC 2A\n");
```
*/

#![no_std]
use ::core::{ptr, mem, fmt, ops, cmp};

#[cfg(test)]
#[macro_use]
extern crate std;

mod lde;
pub mod ext;

//----------------------------------------------------------------

pub trait VirtualAddr: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + ops::Add<Output = Self> + ops::AddAssign {}
impl VirtualAddr for u32 {}
impl VirtualAddr for u64 {}

/// Declares the entry point for an instruction set's length disassembler.
pub trait InsnSet: Sized {
	/// Virtual address type.
	type Va: VirtualAddr;
	/// Length disassembles the given bytes.
	///
	/// Returns `0` on failure.
	fn ld(bytes: &[u8]) -> u32;
	/// Creates an iterator over the opcodes contained within the bytes.
	///
	/// Given a virtual address to keep track of the instruction pointer.
	fn lde(bytes: &[u8], va: Self::Va) -> LDE<Self> {
		LDE {
			bytes: bytes,
			va: va,
		}
	}
	#[doc(hidden)]
	fn iter(bytes: &[u8], va: Self::Va) -> LDE<Self> {
		Self::lde(bytes, va)
	}
	/// Helps with coercing arrays.
	fn code(bytes: &[u8], va: Self::Va) -> (&OpCode, Self::Va) {
		(bytes.into(), va)
	}
	#[doc(hidden)]
	fn as_va(len: usize) -> Self::Va;
}

/// Length disassembler for the `x86` instruction set.
///
/// You'll want to import the `InsnSet` trait too.
#[allow(non_camel_case_types)]
pub struct x86;
impl InsnSet for x86 {
	type Va = u32;
	#[inline]
	fn ld(bytes: &[u8]) -> u32 {
		lde::x86::lde_int(bytes)
	}
	#[inline]
	#[doc(hidden)]
	fn as_va(len: usize) -> u32 {
		len as u32
	}
}

/// Length disassembler for the `x86_64` instruction set.
///
/// You'll want to import [`InsnSet`](trait.InsnSet.html) too.
#[allow(non_camel_case_types)]
pub struct x64;
impl InsnSet for x64 {
	type Va = u64;
	#[inline]
	fn ld(bytes: &[u8]) -> u32 {
		lde::x64::lde_int(bytes)
	}
	#[inline]
	#[doc(hidden)]
	fn as_va(len: usize) -> u64 {
		len as u64
	}
}

//----------------------------------------------------------------

/// Defines a type which can be safely constructed from a byte array of the same size.
///
/// Used to allow reading/writing immediates and displacements.
pub unsafe trait Int: Copy {}
unsafe impl Int for u8 {}
unsafe impl Int for u16 {}
unsafe impl Int for u32 {}
unsafe impl Int for u64 {}
unsafe impl Int for i8 {}
unsafe impl Int for i16 {}
unsafe impl Int for i32 {}
unsafe impl Int for i64 {}

/// Byte slice representing an opcode.
#[derive(Eq, PartialEq, Hash)]
pub struct OpCode([u8]);
impl OpCode {
	#[inline]
	pub fn new(bytes: &[u8]) -> &OpCode {
		bytes.into()
	}
	/// Helps reading immediates and displacements.
	#[inline]
	pub fn read<T: Int>(&self, offset: usize) -> T {
		let bytes = &self[offset..offset + mem::size_of::<T>()];
		let target = bytes.as_ptr() as *const T;
		unsafe { ptr::read(target) }
	}
}
impl<'a> From<&'a [u8]> for &'a OpCode {
	#[inline]
	fn from(bytes: &'a [u8]) -> &'a OpCode {
		unsafe { mem::transmute(bytes) }
	}
}
impl ops::Deref for OpCode {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &[u8] {
		&self.0
	}
}
impl cmp::PartialEq<[u8]> for OpCode {
	#[inline]
	fn eq(&self, other: &[u8]) -> bool {
		self.0.eq(other)
	}
}
impl fmt::Display for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		debug_hex(f, &self.0)
	}
}
impl fmt::Debug for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		display_hex(f, &self.0)
	}
}

//----------------------------------------------------------------

#[doc(hidden)]
pub type LDIter<'a, E> = LDE<'a, E>;

/// Length Disassembler Engine.
///
/// Contains the bytes to be disassembled and generic over the instruction set.
pub struct LDE<'a, E: InsnSet> {
	bytes: &'a [u8],
	/// The current virtual address.
	pub va: E::Va,
}
impl<'a, E: InsnSet> Clone for LDE<'a, E> {
	#[inline]
	fn clone(&self) -> Self {
		LDE {
			bytes: self.bytes,
			va: self.va,
		}
	}
}
impl<'a, E: InsnSet> Copy for LDE<'a, E> {}
impl<'a, E: InsnSet> LDE<'a, E> {
	/// Creates a new instance for a specified instruction set.
	///
	/// # Examples
	///
	/// ```
	/// use lde::{LDE, x64};
	///
	/// let it = LDE::new(x64, b"\x40\x55\x48\x83\xEC\x2A", 0x1000);
	/// assert_eq!(&*it, b"\x40\x55\x48\x83\xEC\x2A");
	/// assert_eq!(it.va, 0x1000);
	/// ```
	#[inline]
	pub fn new(_: E, bytes: &'a [u8], va: E::Va) -> LDE<'a, E> {
		E::lde(bytes, va)
	}
	/// Length disassembles the current location.
	///
	/// # Examples
	///
	/// ```
	/// use lde::{LDE, x64, OpCode};
	///
	/// let it = LDE::new(x64, b"\x40\x55\x48\x83\xEC\x2A", 0x1000);
	/// assert_eq!(it.peek(), Some(OpCode::new(b"\x40\x55")));
	///
	/// // The iterator was not advanced.
	/// // Call `LDE::consume` to manually advance the iterator.
	/// assert_eq!(&*it, b"\x40\x55\x48\x83\xEC\x2A");
	/// assert_eq!(it.va, 0x1000);
	/// ```
	#[inline]
	pub fn peek(&self) -> Option<&'a OpCode> {
		let len = E::ld(self.bytes);
		if len > 0 { Some((&self.bytes[..len as usize]).into()) }
		else { None }
	}
	/// Skips bytes from the input without length disassembling them.
	///
	/// # Examples
	///
	/// ```
	/// use lde::{LDE, x64};
	///
	/// let mut it = LDE::new(x64, b"\x40\x55\x48\x83\xEC\x2A", 0x1000);
	/// it.consume(2);
	/// assert_eq!(&*it, b"\x48\x83\xEC\x2A");
	/// assert_eq!(it.va, 0x1002);
	/// ```
	#[inline]
	pub fn consume(&mut self, n: usize) {
		let n = cmp::min(n, self.bytes.len());
		self.bytes = &self.bytes[n..];
		self.va += E::as_va(n);
	}
}
impl<'a, E: InsnSet> ops::Deref for LDE<'a, E> {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &[u8] {
		self.bytes
	}
}
impl<'a, E: InsnSet> Iterator for LDE<'a, E> {
	type Item = (&'a OpCode, E::Va);
	#[inline]
	fn next(&mut self) -> Option<(&'a OpCode, E::Va)> {
		self.peek().map(|code| {
			let va = self.va;
			self.consume(code.len());
			(code, va)
		})
	}
}
impl<'a, E: InsnSet> fmt::Debug for LDE<'a, E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut it = *self;
		while let Some((bytes, _)) = it.next() {
			try!(debug_hex(f, bytes));
			try!(write!(f, " "));
		}
		display_hex(f, it.bytes)
	}
}
impl<'a, E: InsnSet> fmt::Display for LDE<'a, E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (bytes, _) in *self {
			try!(display_hex(f, bytes));
			try!(write!(f, "\n"));
		}
		Ok(())
	}
}

//----------------------------------------------------------------

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
