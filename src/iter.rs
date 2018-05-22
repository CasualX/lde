use core::{cmp, fmt, ops};
use *;

/// Length disassemble iterator.
pub struct Iter<'a, X: Isa> {
	/// The remaining bytes to length disassemble.
	pub bytes: &'a [u8],
	/// The current virtual address.
	pub va: X::Va,
}

impl<'a, X: Isa> Clone for Iter<'a, X> {
	fn clone(&self) -> Self {
		Iter {
			bytes: self.bytes,
			va: self.va,
		}
	}
}

impl<'a, X: Isa> Iter<'a, X> {
	/// Consumes a number of bytes from the input and returns it as an opcode and its virtual address.
	pub fn consume(&mut self, n: usize) -> (&'a OpCode, X::Va) {
		let n = cmp::min(n, self.bytes.len());
		let (head, tail) = self.bytes.split_at(n);
		let result = (head.into(), self.va);
		self.bytes = tail;
		self.va += X::as_va(n);
		result
	}
}

impl<'a, X: Isa> Iterator for Iter<'a, X> {
	type Item = (&'a OpCode, X::Va);
	fn next(&mut self) -> Option<(&'a OpCode, X::Va)> {
		let len = X::ld(self.bytes);
		if len > 0 {
			Some(self.consume(len as usize))
		}
		else {
			None
		}
	}
}

impl<'a, X: Isa> ops::Deref for Iter<'a, X> {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		self.bytes
	}
}

/// Debug formatter.
///
/// Single line, opcodes grouped with square brackets.
/// Alternate flag to put spaces between the bytes.
impl<'a, X: Isa> fmt::Debug for Iter<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut it = self.clone();
		while let Some((opcode, _)) = it.next() {
			f.write_str("[")?;
			opcode::fmt(f, opcode)?;
			f.write_str("] ")?;
		}
		opcode::fmt(f, it.bytes)
	}
}

/// Display formatter.
///
/// One line per opcode.
/// Alternate flag to put spaces between the bytes.
impl<'a, X: Isa> fmt::Display for Iter<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (opcode, _) in self.clone() {
			opcode::fmt(f, opcode)?;
			f.write_str("\n")?;
		}
		Ok(())
	}
}
