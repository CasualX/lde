/*!
Experimental extensions.
*/

use ::core::{ops, ptr, mem};

use super::Int;

//----------------------------------------------------------------

/// Unstable helper to construct opcodes from bytes.
///
/// ```
/// use lde::ext::{OpCodeBuilder};
///
/// let code = OpCodeBuilder::new(5).write(0, 0x05u8).write(1, 42);
/// assert_eq!(&*code, b"\x05\x2A\x00\x00\x00");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OpCodeBuilder {
	len: u8,
	buf: [u8; 15],
}
impl OpCodeBuilder {
	/// Create a new opcode of specified length, max 15.
	#[inline]
	pub fn new(len: u8) -> OpCodeBuilder {
		OpCodeBuilder {
			len: len,
			buf: [0u8; 15],
		}
	}
	/// Write a value to the opcode buffer at specified offset.
	#[inline]
	pub fn write<T: Int>(mut self, offset: usize, val: T) -> OpCodeBuilder {
		let target = (&mut self.buf[offset..offset + mem::size_of::<T>()]).as_mut_ptr() as *mut T;
		unsafe { ptr::write(target, val); }
		self
	}
}
impl ops::Deref for OpCodeBuilder {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &[u8] {
		&self.buf[..self.len as usize]
	}
}
impl ops::DerefMut for OpCodeBuilder {
	#[inline]
	fn deref_mut(&mut self) -> &mut [u8] {
		&mut self.buf[..self.len as usize]
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn ocbuilder() {
		assert_eq!(&*OpCodeBuilder::new(2).write(0, 0x40u8).write(1, 0x55u8), b"\x40\x55");
		assert_eq!(&*OpCodeBuilder::new(5).write(0, 0xB8u8).write(1, 42), b"\xB8\x2A\x00\x00\x00");
	}
}
