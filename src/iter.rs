use core::{cmp, fmt, ops};
use *;

/// Length disassembler iterator.
///
/// Instances are created by the [`Isa::iter`](trait.Isa.html#method.iter) method.
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
	/// Consumes a number of bytes from the input.
	pub fn consume(&mut self, n: usize) {
		let n = cmp::min(n, self.bytes.len());
		self.bytes = &self.bytes[n..];
		self.va += X::as_va(n);
	}
}

impl<'a, X: Isa> Iterator for Iter<'a, X> {
	type Item = Inst<'a, X>;
	fn next(&mut self) -> Option<Inst<'a, X>> {
		let inst_len = X::inst_len(self.bytes);
		if inst_len.total_len > 0 {
			let n = cmp::min(inst_len.total_len as usize, self.bytes.len());
			let inst = Inst::new(&self.bytes[..n], self.va, inst_len);
			self.consume(n);
			Some(inst)
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
		let mut iter = self.clone();
		while let Some(inst) = iter.next() {
			f.write_str("[")?;
			fmt_bytes(inst.bytes(), b'a', f)?;
			f.write_str("] ")?;
		}
		fmt_bytes(iter.bytes, b'a', f)
	}
}

/// Display formatter.
///
/// One line per opcode.
/// Alternate flag to put spaces between the bytes.
impl<'a, X: Isa> fmt::Display for Iter<'a, X> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for inst in self.clone() {
			fmt_bytes(inst.bytes(), b'a', f)?;
			f.write_str("\n")?;
		}
		Ok(())
	}
}
