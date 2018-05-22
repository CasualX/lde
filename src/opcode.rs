use core::{cmp, fmt, mem, ops, ptr, str};
use super::Int;

/// Byte slice representing an opcode.
#[derive(Eq, PartialEq, Hash)]
pub struct OpCode([u8]);
impl OpCode {
	/// Helps reading immediates and displacements.
	///
	/// # Panics
	///
	/// Panics if `offset..offset + sizeof(T)` is out of bounds.
	pub fn read<T: Int>(&self, offset: usize) -> T {
		let p = self.0[offset..offset + mem::size_of::<T>()].as_ptr() as *const T;
		unsafe { ptr::read_unaligned(p) }
	}
	/// Helps writing immediates and displacements.
	///
	/// # Panics
	///
	/// Panics if `offset..offset + sizeof(T)` is out of bounds.
	pub fn write<T: Int>(&mut self, offset: usize, val: T) -> &mut OpCode {
		let p = self.0[offset..offset + mem::size_of::<T>()].as_mut_ptr() as *mut T;
		unsafe { ptr::write_unaligned(p, val); }
		self
	}
}
impl<'a, T: AsRef<[u8]> + ?Sized> From<&'a T> for &'a OpCode {
	fn from(bytes: &'a T) -> &'a OpCode {
		unsafe { mem::transmute(bytes.as_ref()) }
	}
}
impl<'a, T: AsMut<[u8]> + ?Sized> From<&'a mut T> for &'a mut OpCode {
	fn from(bytes: &'a mut T) -> &'a mut OpCode {
		unsafe { mem::transmute(bytes.as_mut()) }
	}
}
impl<'a> From<&'a OpCode> for &'a [u8] {
	fn from(opcode: &'a OpCode) -> &'a [u8] {
		&opcode.0
	}
}
impl<'a> From<&'a mut OpCode> for &'a mut [u8] {
	fn from(opcode: &'a mut OpCode) -> &'a mut [u8] {
		&mut opcode.0
	}
}
impl ops::Deref for OpCode {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		&self.0
	}
}
impl ops::DerefMut for OpCode {
	fn deref_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}
impl<T: AsRef<[u8]> + ?Sized> cmp::PartialEq<T> for OpCode {
	fn eq(&self, other: &T) -> bool {
		self.0.eq(other.as_ref())
	}
}
impl fmt::Display for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt(f, self)
	}
}
impl fmt::Debug for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt(f, self)
	}
}

pub fn fmt(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
	let mut space = false;
	for &byte in bytes.iter() {
		if space && f.alternate() {
			f.write_str(" ")?;
		}
		space = true;

		let n1 = byte >> 4;
		let c1 = if n1 < 10 { b'0' + n1 } else { b'A' + (n1 - 10) };
		let n2 = byte & 0xf;
		let c2 = if n2 < 10 { b'0' + n2 } else { b'A' + (n2 - 10) };
		let s = [c1, c2];
		f.write_str(unsafe { str::from_utf8_unchecked(&s) })?;
	}
	Ok(())
}
