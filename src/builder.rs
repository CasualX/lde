use core::{cmp, fmt, ops};
use super::{Int, OpCode};

//----------------------------------------------------------------

/// OpCode builder.
///
/// # Examples
///
/// ```
/// let code = lde::OpCodeBuilder::from(b"\xE8****").write(1, 0x01010101u32);
/// assert_eq!(*code, b"\xE8\x01\x01\x01\x01");
/// ```
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct OpCodeBuilder {
	len: u8,
	buf: [u8; 15],
}
impl OpCodeBuilder {
	/// Create a new opcode of specified length, clamped to 15.
	pub fn new(len: usize) -> OpCodeBuilder {
		OpCodeBuilder {
			len: cmp::min(len, 15) as u8,
			buf: [0u8; 15],
		}
	}
	/// Helps writing immediates and displacements.
	///
	/// # Panics
	///
	/// Panics if `offset..offset + sizeof(T)` is out of bounds.
	pub fn write<T: Int>(mut self, offset: usize, val: T) -> OpCodeBuilder {
		(*self).write(offset, val); self
	}
}
/// Converts a template byte slice to an opcode builder.
impl<T: AsRef<[u8]>> From<T> for OpCodeBuilder {
	#[inline]
	fn from(val: T) -> OpCodeBuilder {
		let bytes = val.as_ref();
		let len = cmp::min(bytes.len(), 15);
		let mut builder = OpCodeBuilder::new(len);
		builder.buf[..len].copy_from_slice(&bytes[..len]);
		builder
	}
}
impl ops::Deref for OpCodeBuilder {
	type Target = OpCode;
	fn deref(&self) -> &OpCode {
		(&self.buf[..self.len as usize]).into()
	}
}
impl ops::DerefMut for OpCodeBuilder {
	fn deref_mut(&mut self) -> &mut OpCode {
		(&mut self.buf[..self.len as usize]).into()
	}
}
impl fmt::Debug for OpCodeBuilder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}
impl fmt::Display for OpCodeBuilder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&**self, f)
	}
}

//----------------------------------------------------------------

#[test]
fn ocbuilder() {
	assert_eq!(*OpCodeBuilder::new(2).write(0, 0x40u8).write(1, 0x55u8), b"\x40\x55");
	assert_eq!(*OpCodeBuilder::new(5).write(0, 0xB8u8).write(1, 42), b"\xB8\x2A\x00\x00\x00");
}
