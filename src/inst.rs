/*!
Defines the x86 instruction struct.
 */

use core::{fmt};
use {Isa, fmt_bytes};

/// Instruction length in bytes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InstLen {
	/// Total length of the instruction.
	pub total_len: u8,
	/// Length of the operation code.
	pub op_len: u8,
	/// Number of argument bytes.
	pub arg_len: u8,
	/// Number of prefix bytes.
	pub prefix_len: u8,
}
impl InstLen {
	pub const EMPTY: InstLen = InstLen { total_len: 0, op_len: 0, arg_len: 0, prefix_len: 0 };
}

/// Instruction.
pub struct Inst<'a, X: Isa> {
	bytes: &'a [u8],
	va: X::Va,
	len: InstLen,
}
impl<'a, X: Isa> Copy for Inst<'a, X> {}
impl<'a, X: Isa> Clone for Inst<'a, X> {
	fn clone(&self) -> Inst<'a, X> { *self }
}
impl<'a, X: Isa> Inst<'a, X> {
	pub(crate) fn new(bytes: &'a [u8], va: X::Va, len: InstLen) -> Inst<'a, X> {
		Inst { bytes, va, len }
	}
	/// Gets the instruction bytes.
	pub fn bytes(&self) -> &'a [u8] {
		self.bytes
	}
	/// Gets the bytes part of the instruction prefixes (if any).
	pub fn prefix_bytes(&self) -> &'a [u8] {
		let end = self.len.prefix_len as usize;
		&self.bytes[..end]
	}
	/// Gets the bytes part of the instruction opcode.
	pub fn op_bytes(&self) -> &'a [u8] {
		let start = self.len.prefix_len as usize;
		let end = start + self.len.op_len as usize;
		&self.bytes[start..end]
	}
	/// Gets the bytes part of the instruction arguments.
	pub fn arg_bytes(&self) -> &'a [u8] {
		let end = self.len.total_len as usize;
		let start = end - self.len.arg_len as usize;
		&self.bytes[start..end]
	}
	/// Gets the virtual address
	pub fn va(&self) -> X::Va {
		self.va
	}
}
impl<'a, X: Isa> fmt::Debug for Inst<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::LowerHex::fmt(self, f)
	}
}
impl<'a, X: Isa> fmt::Display for Inst<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::LowerHex::fmt(self, f)
	}
}
impl<'a, X: Isa> fmt::UpperHex for Inst<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt_bytes(self.bytes, b'A', f)
	}
}
impl<'a, X: Isa> fmt::LowerHex for Inst<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt_bytes(self.bytes, b'a', f)
	}
}
