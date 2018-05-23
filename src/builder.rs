use core::{cmp, fmt, ops};
use super::{Int, OpCode};

//----------------------------------------------------------------

/// Opcode builder.
///
/// Conveniently write new opcodes by their byte representation.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct OcBuilder {
	len: u8,
	buf: [u8; 15],
}
impl OcBuilder {
	/// Create a new opcode of specified length, clamped to 15.
	///
	/// The opcode is initialized to all zeroes.
	pub fn new(len: usize) -> OcBuilder {
		OcBuilder {
			len: cmp::min(len, 15) as u8,
			buf: [0; 15],
		}
	}
	/// Helps writing immediate and displacement values.
	///
	/// # Examples
	///
	/// ```
	/// let result = lde::OcBuilder::from(b"\xE8****").write(1, 0x01010101_u32);
	/// assert_eq!(*result, b"\xE8\x01\x01\x01\x01");
	/// ```
	///
	/// # Panics
	///
	/// Panics if `offset..offset + sizeof(T)` is out of bounds.
	pub fn write<T: Int>(mut self, offset: usize, val: T) -> OcBuilder {
		(*self).write(offset, val); self
	}
}
/// Converts a template byte slice to an opcode builder.
///
/// The input length is clamped to 15 bytes.
impl<T: AsRef<[u8]>> From<T> for OcBuilder {
	#[inline]
	fn from(val: T) -> OcBuilder {
		let bytes = val.as_ref();
		let len = cmp::min(bytes.len(), 15);
		let mut builder = OcBuilder::new(len);
		builder.buf[..len].copy_from_slice(&bytes[..len]);
		builder
	}
}
impl ops::Deref for OcBuilder {
	type Target = OpCode;
	fn deref(&self) -> &OpCode {
		(&self.buf[..self.len as usize]).into()
	}
}
impl ops::DerefMut for OcBuilder {
	fn deref_mut(&mut self) -> &mut OpCode {
		(&mut self.buf[..self.len as usize]).into()
	}
}
impl fmt::Debug for OcBuilder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}
impl fmt::Display for OcBuilder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&**self, f)
	}
}

//----------------------------------------------------------------

#[test]
fn ocbuilder() {
	assert_eq!(*OcBuilder::new(2).write(0, 0x40u8).write(1, 0x55u8), b"\x40\x55");
	assert_eq!(*OcBuilder::new(5).write(0, 0xB8u8).write(1, 42), b"\xB8\x2A\x00\x00\x00");
}
