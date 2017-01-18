/*!
Demo generating trampoline using length disassembly.
*/

use ::std::io;

extern crate lde;
use lde::{InsnSet, x86, LDE};

/*
```
:00000000 56         push esi
:00000001 33f6       xor esi,esi
:00000003 57         push edi
:00000004 bfa0104000 mov edi,0x4010a0
:00000009 85d2       test edx,edx
:0000000b 7410       je loc_0000001d
:0000000d 8bf2       mov esi,edx
:0000000f 8bfa       mov edi,edx
```
*/
static INPUT_CODE: &'static [u8] = b"\x56\x33\xF6\x57\xBF\xA0\x10\x40\x00\x85\xD2\x74\x10\x8B\xF2\x8B\xFA";

// Calculate how many bytes need to be copied from the input stream.
// Either you have enough bytes in the input, Ok(bytes) or not, Err(bytes).
pub fn count<I: InsnSet>(stream: LDE<I>, min_bytes: usize) -> Result<usize, usize> {
	let mut written = 0;
	for _ in stream.map(|(opcode, _)| opcode.len()).take_while(|&len| { written += len; written < min_bytes }) {}
	if written >= min_bytes { Ok(written) }
	else { Err(written) }
}

// Generate and relocate the trampoline.
// FIXME! This won't work...
pub fn trampoline<I: InsnSet, W: io::Write>(stream: LDE<I>, buf: &mut W, min_bytes: usize) -> io::Result<()> {
	let mut written = 0;
	let stream = stream.take_while(|&(opcode, _)| {
		written += opcode.len();
		written < min_bytes
	});

	for (opcode, _va) in stream {
		// Relocate the opcode as needed...
		buf.write_all(opcode)?;
	}
	Ok(())
}

fn main() {
	assert_eq!(count(LDE::new(x86, INPUT_CODE, 0x1000), 5), Ok(9));
}
