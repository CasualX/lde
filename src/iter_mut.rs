use core::{cmp, fmt, ops, ptr};
use *;

/// Length disassemble mutable iterator.
pub struct IterMut<'a, X: Isa> {
	pub bytes: &'a mut [u8],
	pub va: X::Va,
}

impl<'a, X: Isa> IterMut<'a, X> {
	/// Cast as const iterator.
	pub fn as_iter<'s>(&'s self) -> Iter<'s, X> {
		Iter {
			bytes: self.bytes,
			va: self.va,
		}
	}
	/// Cast into const iterator.
	pub fn into_iter(self) -> Iter<'a, X> {
		Iter {
			bytes: self.bytes,
			va: self.va,
		}
	}
	/// Consumes a number of bytes from the input and returns it as an opcode and its virtual address.
	pub fn consume(&mut self, n: usize) -> (&'a mut OpCode, X::Va) {
		let n = cmp::min(n, self.bytes.len());
		// The trouble here is that we want ownership of self.bytes, split it up and reinitialize self
		// However this would need a `mem::replace_with` to satisfy the lifetime requirements of the mutable reference
		// Temp fix to use some unsafe code to whack the lifetimes
		let (head, tail) = unsafe { ptr::read(&mut self.bytes) }.split_at_mut(n);
		let result = (head.into(), self.va);
		self.bytes = tail;
		self.va += X::as_va(n);
		result
	}
}

impl<'a, X: Isa> Iterator for IterMut<'a, X> {
	type Item = (&'a mut OpCode, X::Va);
	fn next(&mut self) -> Option<Self::Item> {
		let len = X::ld(self.bytes);
		if len > 0 {
			Some(self.consume(len as usize))
		}
		else {
			None
		}
	}
}

impl<'a, X: Isa> ops::Deref for IterMut<'a, X> {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		self.bytes
	}
}
impl<'a, X: Isa> ops::DerefMut for IterMut<'a, X> {
	fn deref_mut(&mut self) -> &mut [u8] {
		self.bytes
	}
}

/// Debug formatter.
///
/// Single line, opcodes grouped with square brackets.
/// Alternate flag to put spaces between the bytes.
impl<'a, X: Isa> fmt::Debug for IterMut<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iter().fmt(f)
	}
}

/// Display formatter.
///
/// One line per opcode.
/// Alternate flag to put spaces between the bytes.
impl<'a, X: Isa> fmt::Display for IterMut<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iter().fmt(f)
	}
}
