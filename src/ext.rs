/*! Experimental extensions.

*/

use ::core::{ops, ptr, mem};

//----------------------------------------------------------------

/// Unstable helper to construct opcodes from bytes.
///
/// ```
///# use lde::{OpCode, OpCodeBuilder};
/// let _ = OpCode(OpCodeBuilder::new(5).write(0, 0x05u8).write(1, 42));
/// ```
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
			buf: unsafe { mem::uninitialized() },
		}
	}
	/// Write a value to the opcode buffer at specified offset.
	#[inline]
	pub fn write<T: Copy>(&mut self, offset: usize, val: T) -> &mut OpCodeBuilder {
		unsafe { ptr::write((&mut self.buf[offset..offset + mem::size_of::<T>()]).as_mut_ptr() as *mut T, val) };
		self
	}
}
/// Preferably target `OpCode` instead but that is not possible in the current design.
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
