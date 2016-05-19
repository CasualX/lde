Length Disassembler Engine
==========================

Given a byte slice, extract the lengths of the opcodes in it.

Supports `x86` and `x86_64`.

Examples
--------

Get the length of the first opcode in a byte slice.

```rust
use lde::{InsnSet, x64};

assert_eq!(x64::ld(b"\x40\x55\x48\x83\xEC\xFC\x00\x80"), 2);
```

Iterate over the opcodes contained in a byte slice.

```rust
use lde::{InsnSet, OpCode, x64};

assert_eq!(x64::iter(b"\x40\x55\x48\x83\xEC\xFC\x00\x80").collect::<Vec<OpCode>>(),
	vec![OpCode(b"\x40\x55"), OpCode(b"\x48\x83\xEC\xFC")]);
```

License
-------

MIT, see license.txt
